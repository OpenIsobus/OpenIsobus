use alloc::collections::VecDeque;

use crate::{
    iso_11783_3::PDU,
    iso_11783_5::Name,
    iso_11783_7::{LanguageSettings, LanguageSettingsBuilder},
    Isobus, IsobusAddress,
};

use super::{events::EventType, pdu::*, ObjectPool};

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Connected,
    RequestedLanguageCommand,
    RequestedGetHardwareResponse,
    RequestedGetNumberOfSoftkeysResponse,
    RequestedGetTextFontDataResponse,
    RequestedGetVersionsResponse,
    RequestedTimeDate,
    RequestedVTVersion,
    RequestedMemory,
    SendingObjectPool,
    ObjectPoolSend,
}

pub struct WorkingSet {
    state: State,
    isobus: Isobus,
    object_pool: ObjectPool,
    connected_vt: IsobusAddress,
    language_settings: LanguageSettings,

    event_queue: VecDeque<EventType>,

    working_set_maintenance_time: u64,
    is_first_working_set_maintenance: bool,
}

impl WorkingSet {
    pub fn new(object_pool: ObjectPool) -> Self {
        let isobus = Isobus::builder()
            .name(
                Name::builder()
                    .has_self_configurable_address(true) // Dynamicaly claim address
                    .industry_group(2) // Agricultural machinery
                    .device_class(25) // Slurry/Manure Applicators
                    .function(128) // Slurry/Manure Rate Control
                    .manufacturer_code(1407) // Open-Agriculture
                    .ecu_instance(1)
                    .build(),
            )
            // .address_to_claim(IsobusAddress(0x80)) // Address for the in cab VT
            .build();

        Self {
            state: State::Idle,
            isobus,
            object_pool,
            connected_vt: IsobusAddress::NULL,
            language_settings: LanguageSettingsBuilder::new().build(),

            event_queue: VecDeque::new(),

            working_set_maintenance_time: 0,
            is_first_working_set_maintenance: true,
        }
    }

    pub fn process(&mut self, time: u64) {
        let pdus = self.isobus.process(time);

        for pdu in pdus {
            // Received the first VT Status Message
            if pdu.is_vt_status_message()
                && !self.is_vt_connected()
                && self.isobus.is_connected()
                && self.state == State::Idle
            {
                log::info!("Start connecting to VT: {}", pdu.source_address());
                self.connected_vt = pdu.source_address();
                self.isobus.send(
                    PDU::new_working_set_master(self.isobus.claimed_address()),
                    time,
                );

                // Send out the first Working set maintenance message.
                self.cyclic_send_working_set_maintenance_message(time);

                self.isobus.send(
                    PDU::new_request_language_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                    ),
                    time,
                );
                self.state = State::RequestedLanguageCommand;
                break;
            }

            // Received the language command
            if pdu.is_language_command() && self.state == State::RequestedLanguageCommand {
                self.language_settings = LanguageSettings::from_data(&pdu.data::<8>());
                self.isobus.send(
                    PDU::new_get_hardware_message(self.connected_vt, self.isobus.claimed_address()),
                    time,
                );
                self.state = State::RequestedGetHardwareResponse;
                continue;
            }

            // Received the get hardware response
            if pdu.is_get_hardware_response() && self.state == State::RequestedGetHardwareResponse {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_get_number_of_softkeys_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                    ),
                    time,
                );
                self.state = State::RequestedGetNumberOfSoftkeysResponse;
                continue;
            }

            // Received the get number of softkeys response
            if pdu.is_get_number_of_softkeys_response()
                && self.state == State::RequestedGetNumberOfSoftkeysResponse
            {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_get_text_font_data_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                    ),
                    time,
                );
                self.state = State::RequestedGetTextFontDataResponse;
                continue;
            }

            // Received the get text font data response
            if pdu.is_get_text_font_data_response()
                && self.state == State::RequestedGetTextFontDataResponse
            {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_get_versions_message(self.connected_vt, self.isobus.claimed_address()),
                    time,
                );
                self.state = State::RequestedGetVersionsResponse;
                continue;
            }

            // Received the get version response
            if pdu.is_get_versions_response() && self.state == State::RequestedGetVersionsResponse {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_request_time_date(self.connected_vt, self.isobus.claimed_address()),
                    time,
                );
                self.state = State::RequestedTimeDate;
                continue;
            }

            // Received the time/date
            if pdu.is_time_date() && self.state == State::RequestedTimeDate {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_get_memory_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        0,
                    ),
                    time,
                );
                self.state = State::RequestedVTVersion;
                continue;
            }

            // Received get memory response containing the VT version
            if pdu.is_get_memory_response() && self.state == State::RequestedVTVersion {
                // self.language_settings = LanguageSettings::from_data(pdu.data());
                self.isobus.send(
                    PDU::new_get_memory_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        self.object_pool.size(),
                    ),
                    time,
                );
                self.state = State::RequestedMemory;
                continue;
            }

            // Received get memory response and check if there is enough space for our object pool
            if pdu.is_get_memory_response() && self.state == State::RequestedMemory {
                self.isobus.send(
                    PDU::new_object_pool_transfer_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        &self.object_pool,
                    ),
                    time,
                );
                self.state = State::SendingObjectPool;
                continue;
            }

            // Received the result, our own object pool transfer message returned by the tp or etp
            if pdu.is_object_pool_transfer_message() && self.state == State::SendingObjectPool {
                self.isobus.send(
                    PDU::new_end_of_object_pool_message(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                    ),
                    time,
                );
                self.state = State::ObjectPoolSend;
                continue;
            }

            // Received end of object pool response
            if pdu.is_end_of_object_pool_response() && self.state == State::ObjectPoolSend {
                self.state = State::Connected;
                continue;
            }

            if self.state == State::Connected {
                if pdu.is_vt_status_message() {
                    let data: VTStatusMessage = pdu.data_raw().into();

                    if data.active_working_set == self.isobus.claimed_address() {
                        self.event_queue.push_back(EventType::OnActivate);
                    } else {
                        self.event_queue.push_back(EventType::OnDeactivate);
                    }
                }

                if pdu.is_soft_key_activation_message() {
                    let data: SoftKeyActivationMessage = pdu.data_raw().into();

                    match data.key_activation_code {
                        KeyActivationCode::Released => {
                            self.event_queue.push_back(EventType::SoftKeyReleased(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Pressed => {
                            self.event_queue.push_back(EventType::SoftKeyPressed(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Held => {
                            self.event_queue.push_back(EventType::SoftKeyHeld(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Aborted => todo!(),
                    }

                    // Send optional response.
                    self.isobus.send(
                        PDU::new_soft_key_activation_response(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            data.into(),
                        ),
                        time,
                    );
                }

                if pdu.is_button_activation_message() {
                    let data: ButtonActivationMessage = pdu.data_raw().into();

                    match data.key_activation_code {
                        KeyActivationCode::Released => {
                            self.event_queue.push_back(EventType::SoftKeyReleased(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Pressed => {
                            self.event_queue.push_back(EventType::SoftKeyPressed(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Held => {
                            self.event_queue.push_back(EventType::SoftKeyHeld(
                                data.id,
                                data.parent_id,
                                data.key_number,
                            ));
                        }
                        KeyActivationCode::Aborted => todo!(),
                    }

                    // Send optional response.
                    self.isobus.send(
                        PDU::new_button_activation_response(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            data.into(),
                        ),
                        time,
                    );
                }

                if pdu.is_vt_change_numeric_value_command() {
                    let data: VTChangeNumericValueCommand = pdu.data_raw().into();

                    self.event_queue
                        .push_back(EventType::NumericValueChanged(data.id, data.value));

                    // Send optional response.
                    self.isobus.send(
                        PDU::new_vt_change_numeric_value_response(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            data.into(),
                        ),
                        time,
                    );
                }

                if pdu.is_vt_change_string_value_command() {
                    let data: VTChangeStringValueCommand = pdu.data_raw().into();
                    self.event_queue
                        .push_back(EventType::StringValueChanged(data.id, data.value.clone()));

                    // Send optional response.
                    self.isobus.send(
                        PDU::new_vt_change_string_value_response(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            data.into(),
                        ),
                        time,
                    );
                }

                continue;
            }
        }

        // If we no longer have an address claimed, cleanup the workingset state.
        if !self.isobus.is_connected() {
            self.disconnect_vt();
        }

        // Cyclicly send the working set maintenance every second.
        self.cyclic_send_working_set_maintenance_message(time);
    }

    pub fn next_event(&mut self) -> Option<EventType> {
        self.event_queue.pop_front()
    }

    pub fn send_event(&mut self, event: EventType, time: u64) {
        match event {
            EventType::NumericValueChanged(id, value) => {
                let data = ChangeNumericValueCommand { id, value };

                self.isobus.send(
                    PDU::new_change_numeric_value_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        data,
                    ),
                    time,
                );
            }
            EventType::ActiveMaskChanged(working_set_id, mask_id) => {
                let data = ChangeActiveMaskCommand {
                    working_set_id,
                    mask_id,
                };

                self.isobus.send(
                    PDU::new_change_active_mask_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        data,
                    ),
                    time,
                );
            }
            EventType::StringValueChanged(id, value) => {
                let data = ChangeStringValueCommand { id, value };

                self.isobus.send(
                    PDU::new_change_string_value_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        data,
                    ),
                    time,
                );
            }
            _ => {}
        }
    }

    fn is_vt_connected(&mut self) -> bool {
        self.connected_vt != IsobusAddress::NULL
    }

    fn disconnect_vt(&mut self) {
        self.state = State::Idle;
    }

    fn cyclic_send_working_set_maintenance_message(&mut self, time: u64) {
        if time < self.working_set_maintenance_time + 1000 {
            return;
        }

        let bit_mask = if self.is_first_working_set_maintenance {
            self.is_first_working_set_maintenance = false;
            WorkingSetMaintenanceCode::INITIATING
        } else {
            WorkingSetMaintenanceCode::empty()
        };

        let data = WorkingSetMaintenanceMessage {
            bit_mask,
            version_number: VTVersion::V3,
        };

        self.isobus.send(
            PDU::new_working_set_maintenance_message(
                self.connected_vt,
                self.isobus.claimed_address(),
                data,
            ),
            time,
        );

        self.working_set_maintenance_time = time;
    }
}
