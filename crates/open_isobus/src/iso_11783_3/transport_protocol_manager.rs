use alloc::{boxed::Box, collections::VecDeque, vec, vec::Vec};

use crate::{drivers::CanDriverTrait, isobus::IsobusAddress};

use super::{TpAbortReasons, PDU, PGN};

const TP_TIMEOUT_T1: u64 = 750;
const TP_TIMEOUT_T2: u64 = 1250;
const TP_TIMEOUT_T3: u64 = 1750;
const TP_TIMEOUT_T4: u64 = 1050;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Sending,
    Receiving,
}

pub struct TransportProtocolManager {
    backlog: VecDeque<PDU>,
    pdu_to_send: Option<PDU>,
    timeout_time: u64,
    receive_buffer: Vec<u8>,
    receive_pgn: Option<PGN>,
    receive_nr_of_packets: u8,
}

impl TransportProtocolManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, can: &mut Box<dyn CanDriverTrait>, pdu: PDU, time: u64) {
        // Start a new sending connection or
        // if we are already in a connection, store the PDU in the backlog.
        match self.state() {
            State::Idle => {
                self.open_sending_connection(can, pdu, time);
            }
            State::Sending | State::Receiving => {
                self.backlog.push_back(pdu);
            }
        }
    }

    // All pdu's to process are global or ment for us.
    pub fn process(
        &mut self,
        can: &mut Box<dyn CanDriverTrait>,
        claimed_address: IsobusAddress,
        pdu: Option<PDU>,
        time: u64,
    ) -> Option<PDU> {
        // If connected check if a timeout has occurred.
        if self.is_connected() {
            self.process_timeout(can, claimed_address, time);
        }

        // Statements after this need to process a PDU.
        // Return if no pdu was given
        let pdu = match pdu {
            Some(pdu) => pdu,
            None => return None,
        };

        // Received a request to send meant for us.
        if pdu.is_tp_request_to_send() {
            // When idling, accept the request to send and send a clear to send message.
            if self.state() == State::Idle {
                let data: [u8; 8] = pdu.data::<8>();

                let nr_of_bytes: u16 = u16::from_le_bytes([data[1], data[2]]);
                self.receive_buffer = vec![0xFF; nr_of_bytes as usize];
                self.receive_nr_of_packets = u8::min(data[3], data[4]);
                let packet_pgn: PGN = PGN::from_le_bytes([data[5], data[6], data[7]]);

                can.write(
                    PDU::new_tp_clear_to_send(
                        self.receive_nr_of_packets,
                        1,
                        packet_pgn,
                        pdu.source_address(),
                        claimed_address,
                    )
                    .into(),
                );

                self.receive_pgn = Some(packet_pgn);
                self.timeout_time = time + TP_TIMEOUT_T2;
            } else {
                // If we are already in a connection, abort the new connection.
                can.write(
                    PDU::new_tp_connection_abort(
                        TpAbortReasons::AlreadyConnected,
                        pdu.pgn(),
                        pdu.source_address(),
                        claimed_address,
                    )
                    .into(),
                );
            }
        }

        // Received a clear to send meant for us.
        if pdu.is_tp_clear_to_send() && self.state() == State::Sending {
            let data: [u8; 8] = pdu.data::<8>();
            let nr_of_packets = data[1];
            let next_packet = data[2];

            if nr_of_packets == 0 {
                self.timeout_time = time + TP_TIMEOUT_T4;
            } else {
                self.send_pdu_data(
                    can,
                    next_packet,
                    nr_of_packets,
                    pdu.source_address(),
                    claimed_address,
                );
                self.timeout_time = time + TP_TIMEOUT_T3;
            }
        }

        // Received an end of message meant for us.
        if pdu.is_tp_end_of_message_acknowledge() && self.state() == State::Sending {
            let finished_pdu = self.pdu_to_send.take();
            self.close_connection();

            // If we have PDUs in the backlog, start a new connection.
            if let Some(pdu) = self.backlog.pop_front() {
                self.open_sending_connection(can, pdu, time);
            }

            return finished_pdu;
        }

        // Received an data transfer meant for us.
        if pdu.is_tp_data_transfer() && self.state() == State::Receiving {
            let data: [u8; 8] = pdu.data::<8>();
            let packet_nr = data[0];

            for i in 0..7 {
                let index = ((packet_nr - 1) * 7 + i) as usize;
                if index >= self.receive_buffer.len() {
                    break;
                }
                self.receive_buffer[index] = data[(i + 1) as usize];
            }

            self.timeout_time = time + TP_TIMEOUT_T1;

            if self.receive_nr_of_packets == packet_nr {
                let mut finished_pdu = None;

                if let Some(pgn) = self.receive_pgn {
                    can.write(
                        PDU::new_tp_end_of_message_acknowledge(
                            self.receive_buffer.len() as u16,
                            self.receive_nr_of_packets,
                            pgn,
                            pdu.source_address(),
                            claimed_address,
                        )
                        .into(),
                    );

                    if pgn.is_vt_to_ecu() {
                        finished_pdu = Some(PDU::new_vt_to_ecu(
                            pdu.source_address(),
                            claimed_address,
                            self.receive_buffer.clone(),
                        ))
                    }
                }

                // log::info!("{:#?}", finished_pdu);

                self.close_connection();

                return finished_pdu;
            }
        }

        // Received a connection abort meant for us.
        if pdu.is_tp_connection_abort() && self.state() == State::Sending {
            if let Some(pdu) = self.pdu_to_send.take() {
                self.close_connection();
                self.send(can, pdu, time);
            }
        }

        None
    }

    fn process_timeout(
        &mut self,
        can: &mut Box<dyn CanDriverTrait>,
        claimed_address: IsobusAddress,
        time: u64,
    ) {
        if let Some(pdu_to_send) = &self.pdu_to_send {
            if time > self.timeout_time && self.state() != State::Idle {
                can.write(
                    PDU::new_tp_connection_abort(
                        TpAbortReasons::Timeout,
                        pdu_to_send.pgn(),
                        pdu_to_send.destination_address(),
                        claimed_address,
                    )
                    .into(),
                );
                self.close_connection();
            }
        }
    }

    // TODO: Cleanup connection creation
    fn open_sending_connection(&mut self, can: &mut Box<dyn CanDriverTrait>, pdu: PDU, time: u64) {
        let number_of_bytes = pdu.data_len() as u16;
        let number_of_packets = ((number_of_bytes + 7 - 1) / 7) as u8; // Round up using (x+d-1)/d

        if pdu.is_pdu2() {
            can.write(
                PDU::new_tp_broadcast_announce_message(
                    number_of_bytes,
                    number_of_packets,
                    pdu.pgn(),
                    pdu.source_address(),
                )
                .into(),
            );
            self.send_pdu_data(
                can,
                1,
                number_of_packets,
                IsobusAddress::GLOBAL,
                pdu.source_address(),
            );
        } else {
            can.write(
                PDU::new_tp_request_to_send(
                    number_of_bytes,
                    number_of_packets,
                    pdu.pgn(),
                    pdu.destination_address(),
                    pdu.source_address(),
                )
                .into(),
            );
        }

        self.timeout_time = time + TP_TIMEOUT_T3;
        self.pdu_to_send = Some(pdu);
    }

    fn close_connection(&mut self) {
        self.pdu_to_send = None;
        self.timeout_time = u64::MAX;
        self.receive_buffer.clear();
        self.receive_pgn = None;
        self.receive_nr_of_packets = 0;
    }

    fn send_pdu_data(
        &self,
        can: &mut Box<dyn CanDriverTrait>,
        next_packet: u8,
        number_of_packets: u8,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) {
        if let Some(pdu_to_send) = &self.pdu_to_send {
            let chunks: Vec<&[u8]> = pdu_to_send.data_raw().chunks(7).collect();
            for i in 0..number_of_packets {
                can.write(
                    PDU::new_tp_data_transfer(
                        next_packet + i,
                        chunks[(next_packet - 1 + i) as usize],
                        da,
                        sa,
                    )
                    .into(),
                );
            }
        }
    }

    fn state(&self) -> State {
        if self.pdu_to_send.is_some() {
            return State::Sending;
        }

        if self.receive_pgn.is_some() {
            return State::Receiving;
        }

        State::Idle
    }

    fn is_connected(&self) -> bool {
        self.state() == State::Sending || self.state() == State::Receiving
    }
}

impl Default for TransportProtocolManager {
    fn default() -> Self {
        Self {
            backlog: VecDeque::new(),
            pdu_to_send: None,
            timeout_time: u64::MAX,
            receive_buffer: Vec::new(),
            receive_pgn: None,
            receive_nr_of_packets: 0,
        }
    }
}
