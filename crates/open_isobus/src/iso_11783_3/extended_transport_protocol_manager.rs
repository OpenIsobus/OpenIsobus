use alloc::{boxed::Box, collections::VecDeque, vec::Vec};

use crate::{drivers::CanDriverTrait, isobus::IsobusAddress};

use super::{EtpAbortReasons, PDU, PGN};

// const ETP_TIMEOUT_T1: u64 = 750;
// const ETP_TIMEOUT_T2: u64 = 1250;
const ETP_TIMEOUT_T3: u64 = 1750;
const ETP_TIMEOUT_T4: u64 = 1050;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Receiving,
    Sending,
}

pub struct ExtendedTransportProtocolManager {
    backlog: VecDeque<PDU>,
    pdu_to_send: Option<PDU>,
    timeout_time: u64,
    _receive_buffer: Vec<u8>,
    receive_pgn: Option<PGN>,
    _receive_nr_of_packets: u8,
}

impl ExtendedTransportProtocolManager {
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

    pub fn process(
        &mut self,
        can: &mut Box<dyn CanDriverTrait>,
        claimed_address: IsobusAddress,
        pdu: Option<PDU>,
        time: u64,
    ) -> Option<PDU> {
        // If connected and messages are not received on time, send a timeout message and change state.
        if let Some(pdu_to_send) = &self.pdu_to_send {
            if time > self.timeout_time && self.state() != State::Idle {
                can.write(
                    PDU::new_etp_connection_abort(
                        EtpAbortReasons::Timeout,
                        pdu_to_send.pgn(),
                        pdu_to_send.destination_address(),
                        claimed_address,
                    )
                    .into(),
                );
                self.close_connection();
            }
        }

        // Statements after this need to process a PDU.
        let pdu = match pdu {
            Some(pdu) => pdu,
            None => return None,
        };

        // Received a request to send meant for us.
        if pdu.is_etp_request_to_send() && pdu.is_address_specific(claimed_address) {
            // When idling, accept the request to send and send a clear to send message.
            if self.state() == State::Idle {
                //         let data: [u8; 8] = pdu.data::<8>();

                //         let nr_of_bytes: u16 = u16::from_le_bytes([data[1], data[2]]);
                //         self.receive_buffer = vec![0xFF; nr_of_bytes as usize];
                //         self.receive_nr_of_packets = u8::min(data[3], data[4]);
                //         let packet_pgn: PGN = PGN::from_le_bytes([data[5], data[6], data[7]]);

                //         can.write(PDU::new_tp_clear_to_send(
                //             self.receive_nr_of_packets,
                //             1,
                //             packet_pgn,
                //             pdu.source_address(),
                //             claimed_address,
                //         ).into());

                //         self.receive_pgn = Some(packet_pgn);
                //         self.timeout_time = time + TP_TIMEOUT_T2;
                //     } else {
                //         // If we are already in a connection, abort the new connection.
                //         can.write(PDU::new_tp_connection_abort(TpAbortReasons::AlreadyConnected, pdu.pgn(), pdu.source_address(), claimed_address).into());
            }
        }

        // Received a clear to send meant for us.
        if pdu.is_etp_clear_to_send() && pdu.is_address_specific(claimed_address) {
            if self.state() == State::Sending {
                let data: [u8; 8] = pdu.data::<8>();
                let nr_of_packets = data[1];
                let next_packet = u32::from_le_bytes([data[2], data[3], data[4], 0x00]);

                if nr_of_packets == 0 {
                    self.timeout_time = time + ETP_TIMEOUT_T4;
                } else {
                    self.send_pdu_data(
                        can,
                        next_packet,
                        nr_of_packets,
                        pdu.source_address(),
                        claimed_address,
                    );
                    self.timeout_time = time + ETP_TIMEOUT_T3;
                }
            }
        }

        // Received an end of message meant for us.
        if pdu.is_etp_end_of_message_acknowledge() && pdu.is_address_specific(claimed_address) {
            if self.state() == State::Sending {
                let finished_pdu = self.pdu_to_send.take();
                self.close_connection();

                // If we have PDUs in the backlog, start a new connection.
                if let Some(pdu) = self.backlog.pop_front() {
                    self.open_sending_connection(can, pdu, time);
                }

                return finished_pdu;
            }
        }

        // Received an data transfer meant for us.
        // if pdu.is_etp_data_transfer() && pdu.is_address_specific(claimed_address) {
        //     if self.state() == State::Receiving {
        //         let data: [u8; 8] = pdu.data::<8>();
        //         let packet_nr = data[0];

        //         for i in 0..7 {
        //             self.receive_buffer[((packet_nr-1)*7+i) as usize] = data[(i+1) as usize];
        //         }

        //         self.timeout_time = time + TP_TIMEOUT_T1;

        //         if self.receive_nr_of_packets == packet_nr {
        //             if let Some(pgn) = self.receive_pgn {
        //                 can.write(PDU::new_tp_end_of_message_acknowledge(self.receive_buffer.len() as u16, self.receive_nr_of_packets, pgn, pdu.source_address(), claimed_address).into());
        //             }
        //             self.close_connection();
        //         }
        //     }
        // }

        None
    }

    // TODO: Cleanup connection creation
    fn open_sending_connection(&mut self, can: &mut Box<dyn CanDriverTrait>, pdu: PDU, time: u64) {
        let number_of_bytes = pdu.data_len() as u32;

        can.write(
            PDU::new_etp_request_to_send(
                number_of_bytes,
                pdu.pgn(),
                pdu.destination_address(),
                pdu.source_address(),
            )
            .into(),
        );

        self.timeout_time = time + ETP_TIMEOUT_T3;
        self.pdu_to_send = Some(pdu);
    }

    fn close_connection(&mut self) {
        self.pdu_to_send = None;
        self.timeout_time = u64::MAX;
        // self.receive_buffer.clear();
        // self.receive_pgn = None;
        // self.receive_nr_of_packets = 0;
    }

    fn send_pdu_data(
        &self,
        can: &mut Box<dyn CanDriverTrait>,
        next_packet: u32,
        number_of_packets: u8,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) {
        if let Some(pdu_to_send) = &self.pdu_to_send {
            can.write(
                PDU::new_etp_data_packet_offset(
                    number_of_packets,
                    next_packet - 1,
                    pdu_to_send.pgn(),
                    da,
                    sa,
                )
                .into(),
            );

            let chunks: Vec<&[u8]> = pdu_to_send.data_raw().chunks(7).collect();
            for i in 0..number_of_packets {
                can.write(
                    PDU::new_etp_data_transfer(
                        i + 1,
                        chunks[(next_packet - 1 + i as u32) as usize],
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
}

impl Default for ExtendedTransportProtocolManager {
    fn default() -> Self {
        Self {
            backlog: VecDeque::new(),
            pdu_to_send: None,
            timeout_time: u64::MAX,
            _receive_buffer: Vec::new(),
            receive_pgn: None,
            _receive_nr_of_packets: 0,
        }
    }
}
