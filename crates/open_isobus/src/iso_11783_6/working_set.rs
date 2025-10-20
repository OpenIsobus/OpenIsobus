use alloc::{collections::VecDeque, vec::Vec};

use core::{hash::Hasher, u8};
use siphasher::sip::SipHasher;

use crate::{
    Isobus, IsobusAddress,
    iso_11783_3::PDU,
    iso_11783_5::Name,
    iso_11783_7::{LanguageSettings, LanguageSettingsBuilder},
};

use super::{ObjectPool, events::EventType, objects::ObjectId, pdu::*};

const VT_STATUS_MESSAGE_TIMEOUT: u64 = 3000;
const CONNECTION_TIMEOUT: u64 = 30000;
const RECONNECTION_DELAY: u64 = 1000;
const RESPONSE_TIMEOUT: u64 = 1500;
const REQUESTED_VERSIONS_RESPONSE_TIMEOUT: u64 = 5000;
const WORKING_SET_MAINTENANCE_MESSAGE_INTERVAL: u64 = 1000;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Connected,
    Error,
    ReconnectionDelay,
    SendWorkingSetMaster,
    RequestedLanguageCommand,
    RequestedGetHardwareResponse,
    RequestedGetNumberOfSoftkeysResponse,
    RequestedGetTextFontDataResponse,
    // RequestedTimeDate,
    RequestedVTVersion,
    RequestedGetVersionsResponse,
    RequestedLoadObjectPool,
    RequestedStoreObjectPool,
    ReadyToSendObjectPools,
    RequestedMemory,
    SendingObjectPool,
    // SendingLangObjectPool,
    ObjectPoolSend,
    WaitingOnVtStatus(u8),
}

pub struct WorkingSet<'a> {
    state: State,
    isobus: Isobus<'a>,
    // object_pools: BTreeMap<(String, u16, u16), ObjectPool>,
    object_pools: VecDeque<ObjectPool>,
    object_pool_version_label: [u8; 7],
    vt_size: u16,
    softkey_size: u8,
    nr_of_softkeys: u8,
    preferred_vt: IsobusAddress,
    available_vt: VecDeque<IsobusAddress>,
    connected_vt: IsobusAddress,
    language_settings: LanguageSettings,
    active_data_mask: ObjectId,

    event_queue: VecDeque<EventType>,

    working_set_maintenance_time: u64,
    is_first_working_set_maintenance: bool,
    timeout_time: u64,
    reconnection_timeout_time: u64,
    retry_count: u8,
}

impl<'a> WorkingSet<'a> {
    pub fn new(
        can_interface: &'a str,
        // object_pools: BTreeMap<(String, u16, u16), ObjectPool>,
        name: Name,
        preferred_vt: IsobusAddress,
    ) -> Self {
        let isobus: Isobus<'a> = Isobus::builder()
            .name(name)
            // .address_to_claim(IsobusAddress(0x80))
            .can_interface(can_interface)
            .build();

        Self {
            state: State::Idle,
            isobus,
            object_pools: VecDeque::new(),
            object_pool_version_label: [0x20; 7],
            vt_size: 0,
            softkey_size: 0,
            nr_of_softkeys: 0,
            preferred_vt,
            available_vt: VecDeque::new(),
            connected_vt: IsobusAddress::NULL,
            language_settings: LanguageSettingsBuilder::new().build(),
            active_data_mask: ObjectId::NULL,

            event_queue: VecDeque::new(),

            working_set_maintenance_time: u64::MAX,
            is_first_working_set_maintenance: true,
            timeout_time: u64::MAX,
            reconnection_timeout_time: u64::MAX,
            retry_count: u8::MAX,
        }
    }

    pub fn process(&mut self, time: u64) {
        let pdus = self.isobus.process(time);

        for pdu in pdus {
            // Check the ISOBUS for connected VTs
            if pdu.is_vt_status_message() && pdu.source_address() != self.connected_vt {
                if !self.available_vt.contains(&pdu.source_address()) {
                    self.available_vt.push_back(pdu.source_address());
                    log::info!(
                        "Detected VT: {0} ({0:#02X}) {1:?}",
                        pdu.source_address(),
                        self.available_vt,
                    );
                }
            }

            // Received a VT Status Message
            if pdu.is_vt_status_message()
                && !self.is_vt_connected()
                && self.isobus.is_connected()
                && self.state == State::Idle
            {
                // If the message was from the preferred VT,
                // or the preferred VT address was set to NULL or 0, start connecting.
                // If it was not from the preferred VT, start the connection timeout
                if pdu.source_address() == self.preferred_vt
                    || self.preferred_vt == IsobusAddress::NULL
                    || self.preferred_vt == IsobusAddress(0)
                {
                    log::info!(
                        "Start connecting to VT: {0} ({0:#02X})",
                        pdu.source_address()
                    );

                    self.connected_vt = pdu.source_address();

                    self.isobus.send(
                        PDU::new_working_set_master(self.isobus.claimed_address()),
                        time,
                    );
                    self.state = State::SendWorkingSetMaster;

                    // Send the first working set maintenance message.
                    self.cyclic_send_working_set_maintenance_message(time);

                    // Set the response timeout
                    self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                    self.isobus.send(
                        PDU::new_request_language_command(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                        ),
                        time,
                    );
                    self.state = State::RequestedLanguageCommand;
                } else {
                    if self.timeout_time == u64::MAX {
                        // Start the connection timeout after every new VT
                        self.set_timeout(time, CONNECTION_TIMEOUT, 0);
                        log::info!("Waiting on preferred VT: {0} ({0:#02X})", self.preferred_vt);
                    }
                }
                continue;
            }

            // Received the language command
            if pdu.is_language_command() && self.state == State::RequestedLanguageCommand {
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

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
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                let hardware = GetHardwareResponse::from(pdu.data_raw());
                self.vt_size = hardware.x_size;
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
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

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
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                // TODO: Checking time/date is not needed for now
                // self.isobus.send(
                //     PDU::new_request_time_date(self.connected_vt, self.isobus.claimed_address()),
                //     time,
                // );
                // self.state = State::RequestedTimeDate;

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

            // TODO: Checking time/date is not needed for now
            // Received the time/date
            // if (pdu.is_time_date() || pdu.is_acknowledgement()) // TODO: check if a NACK for the specific request
            //     && self.state == State::RequestedTimeDate
            // {
            //     // Set the response timeout
            //     self.set_timeout(time, RESPONSE_TIMEOUT);

            //     self.isobus.send(
            //         PDU::new_get_memory_message(
            //             self.connected_vt,
            //             self.isobus.claimed_address(),
            //             0,
            //         ),
            //         time,
            //     );
            //     self.state = State::RequestedVTVersion;
            //     continue;
            // }

            // Received get memory response containing the VT version
            if pdu.is_get_memory_response() && self.state == State::RequestedVTVersion {
                // Set the response timeout
                self.set_timeout(time, REQUESTED_VERSIONS_RESPONSE_TIMEOUT, 2); // TODO: measure time taken on Smart430

                let memory_response = GetMemoryResponse::from(pdu.data_raw());
                log::info!("VT version: {}", memory_response.version_number);
                self.event_queue
                    .push_back(EventType::ReadyToSendObjectPool {
                        vt_size: self.vt_size,
                        softkey_size: self.softkey_size,
                        nr_of_softkeys: self.nr_of_softkeys,
                        language_settings: self.language_settings.clone(),
                    });

                self.isobus.send(
                    PDU::new_get_versions_message(self.connected_vt, self.isobus.claimed_address()),
                    time,
                );
                self.state = State::RequestedGetVersionsResponse; // TODO: Timeout on response
                continue;
            }

            // Received the get version response
            // We expect the list with object pools to be filled with the correct pools for the VT hardware
            if pdu.is_get_versions_response() && self.state == State::RequestedGetVersionsResponse {
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                let versions_response = GetVersionsResponse::from(pdu.data_raw());
                let version_labels: Vec<&str> = versions_response
                    .version_labels
                    .iter()
                    .map(|label| alloc::str::from_utf8(label))
                    .flatten()
                    .collect();

                log::info!("OP versions: {:?}", version_labels);

                // Calculate the hash of all object pools
                let mut hasher = SipHasher::new();
                for object_pool in &self.object_pools {
                    hasher.write(&object_pool.as_iop());
                }
                let hash = hasher.finish();

                // Convert the hash to a base-16 string and take the first 7 bytes
                use numtoa::NumToA;
                let mut buffer: [u8; 16] = [0x20; 16];
                let _ = hash.numtoa(16, &mut buffer);
                self.object_pool_version_label.copy_from_slice(&buffer[..7]);

                log::info!(
                    "OP label: \"{:X}\" -> {:?}",
                    hash,
                    alloc::str::from_utf8(&self.object_pool_version_label)
                );

                // If the objectpool is stored, load it from VT memory
                if versions_response
                    .version_labels
                    .contains(&self.object_pool_version_label)
                {
                    self.isobus.send(
                        PDU::new_load_version_command(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            &self.object_pool_version_label,
                        ),
                        time,
                    );
                    self.state = State::RequestedLoadObjectPool;
                } else {
                    // If the objectpool is not stored, start to send the object pools
                    self.state = State::ReadyToSendObjectPools;
                }

                continue;
            }

            // Ready to send object pool.
            // If pools in queue, send them, else, done sending object pools.
            if self.state == State::ReadyToSendObjectPools {
                // Set the response timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                if let Some(object_pool) = self.object_pools.front() {
                    self.isobus.send(
                        PDU::new_get_memory_message(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                            object_pool.size(),
                        ),
                        time,
                    );
                    log::info!("Requested Memory: {} bytes", object_pool.size());
                    self.state = State::RequestedMemory;
                } else {
                    // All object pools send.
                    self.isobus.send(
                        PDU::new_end_of_object_pool_message(
                            self.connected_vt,
                            self.isobus.claimed_address(),
                        ),
                        time,
                    );
                    self.state = State::ObjectPoolSend;
                }
                continue;
            }

            // Received get memory response and check if there is enough space for our object pool
            if pdu.is_get_memory_response() && self.state == State::RequestedMemory {
                let memory_response = GetMemoryResponse::from(pdu.data_raw());
                log::info!("GetMemoryResponse status: {}", memory_response.status);

                // Check the status
                if memory_response.status == 0 {
                    if let Some(object_pool) = self.object_pools.pop_front() {
                        // Update timeout (Timeout after 1ms per 4 bytes with a minimum of 1000ms)
                        self.set_timeout(time, u64::max(object_pool.size() as u64 / 4, 1000), 2);

                        self.isobus.send(
                            PDU::new_object_pool_transfer_message(
                                self.connected_vt,
                                self.isobus.claimed_address(),
                                &object_pool,
                            ),
                            time,
                        );
                        self.state = State::SendingObjectPool;
                    } else {
                        unreachable!(
                            "This should be unreachable because of the check in State::ReadyToSendObjectPools"
                        )
                    }
                } else {
                    log::error!("Not enough memory!");
                    self.state = State::Error;
                }
                continue;
            }

            // Received the result of our object pool transfer message returned by the tp or etp
            if pdu.is_object_pool_transfer_message() && self.state == State::SendingObjectPool {
                // Reset the timeout
                self.set_timeout(0, u64::MAX, u8::MAX);

                // Ready to send more object pools
                self.state = State::ReadyToSendObjectPools;
                continue;
            }

            // Received the result of our object pool transfer message returned by the tp or etp
            if pdu.is_end_of_object_pool_response() && self.state == State::ObjectPoolSend {
                // Reset the timeout
                self.set_timeout(time, RESPONSE_TIMEOUT, 2);

                self.isobus.send(
                    PDU::new_store_version_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        &self.object_pool_version_label,
                    ),
                    time,
                );

                self.state = State::RequestedStoreObjectPool;
                continue;
            }

            // Received a store or load object pool response
            if self.state == State::RequestedStoreObjectPool
                || (pdu.is_load_version_response() && self.state == State::RequestedLoadObjectPool)
            {
                // Update timeout
                self.set_timeout(time, VT_STATUS_MESSAGE_TIMEOUT, 0);

                self.state = State::WaitingOnVtStatus(0);
                continue;
            }

            // Received 3 vt status messages to make sure parsing is finished
            if pdu.is_vt_status_message() && pdu.source_address() == self.connected_vt {
                if let State::WaitingOnVtStatus(_) = self.state {
                    // Update timeout
                    self.set_timeout(time, VT_STATUS_MESSAGE_TIMEOUT, 0);

                    if let State::WaitingOnVtStatus(i) = self.state {
                        let status: VTStatusMessage = pdu.data_raw().into();
                        if !status
                            .vt_busy_code
                            .contains(VTBusyCode::PARSING_OBJECT_POOL)
                        {
                            self.state = if i < 1 {
                                State::WaitingOnVtStatus(i + 1)
                            } else {
                                log::info!("Connected to VT: {0} ({0:#02X})", self.connected_vt);
                                self.event_queue.push_back(EventType::OnConnect {
                                    connected_vt: self.connected_vt,
                                });
                                State::Connected
                            };
                        }
                        continue;
                    }
                }
            }

            if self.state == State::Connected {
                if pdu.is_vt_status_message() && pdu.source_address() == self.connected_vt {
                    // Update timeout
                    self.set_timeout(time, VT_STATUS_MESSAGE_TIMEOUT, 0);

                    let data: VTStatusMessage = pdu.data_raw().into();

                    // If we are the connected working set
                    if data.active_working_set == self.isobus.claimed_address() {
                        // If the active Data mask has changed, queue a ActiveMaskChanged event
                        if self.active_data_mask != data.data_alarm_mask {
                            self.event_queue.push_back(EventType::ActiveMaskChanged(
                                0.into(), // TODO: We don't know the workingset ObjectId, can we get it from somewhere?
                                data.data_alarm_mask.into(),
                            ));
                            self.active_data_mask = data.data_alarm_mask;
                        }

                        // Queue a VTStatus event
                        self.event_queue.push_back(EventType::VTStatus {
                            active_data_mask: data.data_alarm_mask,
                            active_softkey_mask: data.soft_key_mask,
                        });
                    }

                    if data.active_working_set == self.isobus.claimed_address() {
                        self.event_queue.push_back(EventType::OnActivate);
                    } else {
                        self.event_queue.push_back(EventType::OnDeactivate);
                    }
                    continue;
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
                        KeyActivationCode::Aborted => {}
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
                    continue;
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
                        KeyActivationCode::Aborted => {}
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
                    continue;
                }

                if pdu.is_vt_change_numeric_value_message() {
                    let data: VTChangeNumericValueMessage = pdu.data_raw().into();

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
                    continue;
                }

                if pdu.is_vt_change_string_value_message() {
                    let data: VTChangeStringValueMessage = pdu.data_raw().into();
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
                    continue;
                }

                if pdu.is_wheel_based_speed_and_distance() {
                    self.event_queue
                        .push_back(EventType::WheelBasedSpeedAndDistance(pdu.data_raw().into()));
                    continue;
                }

                if pdu.is_ground_based_speed_and_distance() {
                    self.event_queue
                        .push_back(EventType::GroundBasedSpeedAndDistance(
                            pdu.data_raw().into(),
                        ));
                    continue;
                }
            }

            if self.state == State::ReconnectionDelay {
                // If it is the first call in this state, start the timeout
                if self.reconnection_timeout_time == u64::MAX {
                    self.reconnection_timeout_time = time.saturating_add(RECONNECTION_DELAY);
                }

                // If the timeout has elapsed, switch to idle
                if time > self.reconnection_timeout_time {
                    self.state = State::Idle;
                }
            }

            if pdu.is_all_implements_stop_operations() {
                let data = pdu.data_raw();
                self.event_queue
                    .push_back(EventType::AllImplementsStopOperations(
                        data[6],
                        data[7] & 0x03,
                    ));
                continue;
            }

            if pdu.is_delete_object_pool_response() {
                self.event_queue.push_back(EventType::DeleteObjectPool);
                self.disconnect_vt();
                continue;
            }
        }

        // Timeout
        if time > self.timeout_time {
            if self.state == State::RequestedGetVersionsResponse {
                self.state = State::ReadyToSendObjectPools;
            } else {
                match self.state {
                    State::Idle => {
                        log::warn!(
                            "Preferred VT: {0} ({0:#02X}) not available...",
                            self.preferred_vt
                        );
                    }
                    // If we are connected to a VT, timeout if we dont receive a VT status message.
                    State::Connected => log::warn!("VT status message timeout..."),
                    State::Error => log::warn!("Error while connecting, trying again..."),
                    State::ReconnectionDelay => {}
                    State::SendWorkingSetMaster => log::warn!("Send Working Set Master timeout..."),
                    State::RequestedLanguageCommand => {
                        log::warn!("Requested Language Command timeout...")
                    }
                    State::RequestedGetHardwareResponse => {
                        log::warn!("Requested Hardware Response timeout...")
                    }
                    State::RequestedGetNumberOfSoftkeysResponse => {
                        log::warn!("Requested Number Of Softkeys Response timeout...")
                    }
                    State::RequestedGetTextFontDataResponse => {
                        log::warn!("Requested Text Font Data Response timeout...")
                    }
                    // State::RequestedTimeDate => log::warn!("Requested Time Date timeout..."),
                    State::RequestedVTVersion => log::warn!("Requested VT Version timeout..."),
                    State::RequestedGetVersionsResponse => {
                        log::warn!("Requested Versions Response timeout...")
                    }
                    State::RequestedLoadObjectPool => {
                        log::warn!("Requested Load Object Pool timeout...")
                    }
                    State::RequestedStoreObjectPool => {
                        log::warn!("Requested Store Object Pool timeout...")
                    }
                    State::ReadyToSendObjectPools => {
                        log::warn!("Ready To Send Object Pools timeout...")
                    }
                    State::RequestedMemory => log::warn!("Requested Memory timeout..."),
                    State::SendingObjectPool => log::warn!("Sending Object Pool timeout..."),
                    State::ObjectPoolSend => log::warn!("Object Pool Send timeout..."),
                    State::WaitingOnVtStatus(_) => log::warn!("Waiting On Vt Status timeout..."),
                }

                self.disconnect_vt();
            }
        }

        // If we no longer have an address claimed, cleanup the workingset state.
        if !self.isobus.is_connected() && self.is_vt_connected() {
            self.disconnect_vt();
        }

        // Cyclicly send the working set maintenance every second.
        self.cyclic_send_working_set_maintenance_message(time);
    }

    // Setup the timeout and retry count
    fn set_timeout(&mut self, time: u64, timeout: u64, retries: u8) {
        // log::debug!("Set Timeout: {timeout} ({0}:{1})", time, time.saturating_add(timeout));
        self.timeout_time = time.saturating_add(timeout);
        self.retry_count = retries;
    }

    pub fn next_event(&mut self) -> Option<EventType> {
        self.event_queue.pop_front()
    }

    pub fn send_event(&mut self, event: EventType, time: u64) {
        match event {
            EventType::HideShowObject(id, value) => {
                self.isobus.send(
                    PDU::new_hide_show_object_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        HideShowObjectCommand { id, value },
                    ),
                    time,
                );
            }
            EventType::EnableDisableObject(id, value) => {
                self.isobus.send(
                    PDU::new_enable_disable_object_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        EnableDisableObjectCommand { id, value },
                    ),
                    time,
                );
            }
            EventType::NumericValueChanged(id, value) => {
                self.isobus.send(
                    PDU::new_change_numeric_value_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeNumericValueCommand { id, value },
                    ),
                    time,
                );
            }
            EventType::ActiveMaskChanged(working_set_id, mask_id) => {
                self.isobus.send(
                    PDU::new_change_active_mask_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeActiveMaskCommand {
                            working_set_id,
                            mask_id,
                        },
                    ),
                    time,
                );
            }
            EventType::AttributeChanged(object_id, attribute_id, attribute) => {
                self.isobus.send(
                    PDU::new_change_attribute_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeAttributeCommand {
                            object_id,
                            attribute_id,
                            attribute,
                        },
                    ),
                    time,
                );
            }
            EventType::ListItemChanged {
                object_id,
                list_index,
                new_item_id,
            } => {
                self.isobus.send(
                    PDU::new_change_list_item_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeListItemCommand {
                            object_id,
                            list_index,
                            new_item_id,
                        },
                    ),
                    time,
                );
            }
            EventType::DeleteObjectPool => {
                self.isobus.send(
                    PDU::new_delete_object_pool_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        DeleteObjectPoolCommand {},
                    ),
                    time,
                );
            }
            EventType::StringValueChanged(id, value) => {
                self.isobus.send(
                    PDU::new_change_string_value_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeStringValueCommand { id, value },
                    ),
                    time,
                );
            }
            EventType::ChangeChildPosition {
                parent_id,
                object_id,
                new_x,
                new_y,
            } => {
                self.isobus.send(
                    PDU::new_change_child_position_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        ChangeChildPositionCommand {
                            parent_id,
                            object_id,
                            new_x,
                            new_y,
                        },
                    ),
                    time,
                );
            }
            EventType::OnDisconnect {} => {
                self.isobus.send(
                    PDU::new_delete_object_pool_command(
                        self.connected_vt,
                        self.isobus.claimed_address(),
                        DeleteObjectPoolCommand {},
                    ),
                    time,
                );
            }
            _ => {}
        }
    }

    pub fn send_object_pool(&mut self, object_pools: &[ObjectPool]) {
        for object_pool in object_pools {
            self.object_pools.push_back(object_pool.clone());
        }
    }

    fn is_vt_connected(&mut self) -> bool {
        self.state == State::Connected
    }

    fn disconnect_vt(&mut self) {
        // If the retries is 0, disconnect. Otherwise try again.
        if self.retry_count == 0 {
            // If previously connected, log the disconnection and wait before reconnecting
            if self.connected_vt != IsobusAddress::NULL {
                log::info!("Disconnected from VT: {0} ({0:#02X})", self.connected_vt);
                self.state = State::ReconnectionDelay;
            } else {
                self.state = State::Idle;
            }

            // Remove the current VT from the available ones and choose the next one
            self.available_vt.retain(|val| *val != self.preferred_vt);
            if let Some(address) = self.available_vt.pop_front() {
                self.preferred_vt = address;
                log::info!("Preferred VT set to: {0} ({0:#02X})", self.preferred_vt);
            }

            self.object_pools.clear();
            self.object_pool_version_label.fill(0x20);
            self.vt_size = 0;
            self.softkey_size = 0;
            self.nr_of_softkeys = 0;
            self.connected_vt = IsobusAddress::NULL;
            self.language_settings = LanguageSettingsBuilder::new().build();
            self.active_data_mask = ObjectId::NULL;
            self.working_set_maintenance_time = u64::MAX;
            self.is_first_working_set_maintenance = true;
            self.timeout_time = u64::MAX;
            self.reconnection_timeout_time = u64::MAX;
            self.retry_count = u8::MAX;

            self.event_queue.push_back(EventType::OnDisconnect);
        } else {
            self.retry_count -= 1;
        }
    }

    fn cyclic_send_working_set_maintenance_message(&mut self, time: u64) {
        // Exit the function if we have not reached the timeout or are in an Idle state
        // But always run this function if we are in the SendWorkingSetMaster state (The first time we send)
        if (time
            < self
                .working_set_maintenance_time
                .saturating_add(WORKING_SET_MAINTENANCE_MESSAGE_INTERVAL)
            || self.state == State::Idle)
            && self.state != State::SendWorkingSetMaster
        {
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
