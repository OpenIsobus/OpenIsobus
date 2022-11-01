
/// Struct containing all Function ID's
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FunctionEnum {
    Global(GlobalFunctionEnum),
    OnHighwayEquipment(OnHighwayEquipmentFunctionEnum),
    AgriculturalAndForestryEquipment(AgriculturalAndForestryEquipmentFunctionEnum),
    ConstructionEquipment(ConstructionEquipmentFunctionEnum),
    MarineEquipment(MarineEquipmentFunctionEnum),
    IndustrialProcessControl(IndustrialProcessControlFunctionEnum),
}

impl core::fmt::Display for FunctionEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<FunctionEnum> for u8 {
    fn from(value: FunctionEnum) -> Self {
        match value {
            Global(value) => value as u8,
            OnHighwayEquipment(value) => value as u8,
            AgriculturalAndForestryEquipment(value) => value as u8,
            ConstructionEquipment(value) => value as u8,
            MarineEquipment(value) => value as u8,
            IndustrialProcessControl(value) => value as u8,
        }
    }
}


/// Struct containing all Global Function ID's
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum GlobalFunctionEnum {
    Engine = 0,
    AuxiliaryPowerUnit = 1,
    ElectricPropulsionControl = 2,
    Transmission = 3,
    BatteryPackMonitor = 4,
    ShiftControlConsole = 5,
    PowerTakeOff = 6,
    AxleSteering = 7,
    AxleDrive = 8,
    BrakesSystemController = 9,
    BrakesSteerAxle = 10,
    BrakesDriveAxle = 11,
    RetarderEngine = 12,
    RetarderDriveline = 13,
    CruiseControl = 14,
    FuelSystem = 15,
    SteeringController = 16,
    SuspensionSteerAxle = 17,
    SuspensionDriveAxle = 18,
    InstrumentCluster = 19,
    TripRecorder = 20,
    CabClimateControl = 21,
    AerodynamicControl = 22,
    VehicleNavigation = 23,
    VehicleSecurity = 24,
    NetworkInterconnectECU = 25,
    BodyController = 26,
    PowerTakeOff = 27,
    OffVehicleGateway = 28,
    VirtualTerminal = 29,
    ManagementComputer = 30,
    PropulsionBatteryCharger = 31,
    HeadwayController = 32,
    SystemMonitor = 33,
    HydraulicPumpController = 34,
    SuspensionSystemController = 35,
    PneumaticSystemController = 36,
    CabController = 37,
    TirePressureControl = 38,
    IgnitionControlModule = 39,
    SeatControl = 40,
    LightingOperatorControls = 41,
    WaterPumpControl = 42,
    TransmissionDisplay = 43,
    ExhaustEmissionControl = 44,
    VehicleDynamicStabilityControl = 45,
    OilSensorUnit = 46,
    InformationSystemController = 47,
    RampControl = 48,
    ClutchConverterControl = 49,
    AuxiliaryHeater = 50,
    ForwardLookingCollisionWarningSystem = 51,
    ChassisController = 52,
    AlternatorChargingSystem = 53,
    CommunicationsUnitCellular = 54,
    CommunicationsUnitSatellite = 55,
    CommunicationsUnitRadio = 56,
    SteeringColumnUnit = 57,
    FanDriveControl = 58,
    Starter = 59,
    CabDisplay = 60,
    FileServerPrinter = 61,
    OnBoardDiagnosticUnit = 62,
    EngineValveController = 63,
    EnduranceBraking = 64,
    GasFlowMeasurement = 65,
    IOController = 66,
    ElectricalSystemController = 67,
    AfterTreatmentSystemGasMeasurement = 68,
    EngineEmissionAfterTreatmentSystem = 69,
    AuxiliaryRegenerationDevice = 70,
    TransferCaseControl = 71,
    CoolantValveController = 72,
    RolloverDetectionControl = 73,
    LubricationSystem = 74,
    SupplementalFan = 75,
    TemperatureSensor = 76,
    FuelPropertiesSensor = 77,
    FireSuppressionSystem = 78,
    PowerSystemsManager = 79,
    ElectricPowertrain = 80,
    HydraulicPowertrain = 81,
    FileServer = 82,
    Printer = 83,
    StartAidDevice = 84,
    EngineInjectionControlModule = 85,
    EVCommunicationController = 86,
    DriverImpairmentDevice = 87,
    ElectricPowerConverter = 88,
    SupplyEquipmentCommunicationController = 89,
    VehicleAdapterCommunicationController = 90,
    Reserved = 128,
    OffBoardDiagnosticServiceTool = 129,
    OnBoardDataLogger = 130,
    PCKeyboard = 131,
    SafetyRestraintSystem = 132,
    Turbocharger = 133,
    GroundBasedSpeedSensor = 134,
    Keypad = 135,
    HumiditySensor = 136,
    ThermalManagementSystemController = 137,
    BrakeStrokeAlert = 138,
    OnBoardAxleGroupScale = 139,
    OnBoardAxleGroupDisplay = 140,
    BatteryCharger = 141,
    TurbochargerCompressorBypass = 142,
    TurbochargerWastegate = 143,
    Throttle = 144,
    InertialSensor = 145,
    FuelActuator = 146,
    EngineExhaustGasRecirculation = 147,
    EngineExhaustBackpressure = 148,
    OnBoardBinWeighingScale = 149,
    OnBoardBinWeighingScaleDisplay = 150,
    EngineCylinderPressureMonitoringSystem = 151,
    ObjectDetection = 152,
    ObjectDetectionDisplay = 153,
    ObjectDetectionSensor = 154,
    PersonnelDetectionDevice = 155,
    #[default] NotAvailable = 255,
}

impl core::fmt::Display for GlobalFunctions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for GlobalFunctions {
    fn from(value: u8) -> Self {
        match value {
            0 => GlobalFunctionEnum::Engine,
            1 => GlobalFunctionEnum::AuxiliaryPowerUnit,
            2 => GlobalFunctionEnum::ElectricPropulsionControl,
            3 => GlobalFunctionEnum::Transmission,
            4 => GlobalFunctionEnum::BatteryPackMonitor,
            5 => GlobalFunctionEnum::ShiftControlConsole,
            6 => GlobalFunctionEnum::PowerTakeOff,
            7 => GlobalFunctionEnum::AxleSteering,
            8 => GlobalFunctionEnum::AxleDrive,
            9 => GlobalFunctionEnum::BrakesSystemController,
            10 => GlobalFunctionEnum::BrakesSteerAxle,
            11 => GlobalFunctionEnum::BrakesDriveAxle,
            12 => GlobalFunctionEnum::RetarderEngine,
            13 => GlobalFunctionEnum::RetarderDriveline,
            14 => GlobalFunctionEnum::CruiseControl,
            15 => GlobalFunctionEnum::FuelSystem,
            16 => GlobalFunctionEnum::SteeringController,
            17 => GlobalFunctionEnum::SuspensionSteerAxle,
            18 => GlobalFunctionEnum::SuspensionDriveAxle,
            19 => GlobalFunctionEnum::InstrumentCluster,
            20 => GlobalFunctionEnum::TripRecorder,
            21 => GlobalFunctionEnum::CabClimateControl,
            22 => GlobalFunctionEnum::AerodynamicControl,
            23 => GlobalFunctionEnum::VehicleNavigation,
            24 => GlobalFunctionEnum::VehicleSecurity,
            25 => GlobalFunctionEnum::NetworkInterconnectECU,
            26 => GlobalFunctionEnum::BodyController,
            27 => GlobalFunctionEnum::PowerTakeOff,
            28 => GlobalFunctionEnum::OffVehicleGateway,
            29 => GlobalFunctionEnum::VirtualTerminal,
            30 => GlobalFunctionEnum::ManagementComputer,
            31 => GlobalFunctionEnum::PropulsionBatteryCharger,
            32 => GlobalFunctionEnum::HeadwayController,
            33 => GlobalFunctionEnum::SystemMonitor,
            34 => GlobalFunctionEnum::HydraulicPumpController,
            35 => GlobalFunctionEnum::SuspensionSystemController,
            36 => GlobalFunctionEnum::PneumaticSystemController,
            37 => GlobalFunctionEnum::CabController,
            38 => GlobalFunctionEnum::TirePressureControl,
            39 => GlobalFunctionEnum::IgnitionControlModule,
            40 => GlobalFunctionEnum::SeatControl,
            41 => GlobalFunctionEnum::LightingOperatorControls,
            42 => GlobalFunctionEnum::WaterPumpControl,
            43 => GlobalFunctionEnum::TransmissionDisplay,
            44 => GlobalFunctionEnum::ExhaustEmissionControl,
            45 => GlobalFunctionEnum::VehicleDynamicStabilityControl,
            46 => GlobalFunctionEnum::OilSensorUnit,
            47 => GlobalFunctionEnum::InformationSystemController,
            48 => GlobalFunctionEnum::RampControl,
            49 => GlobalFunctionEnum::ClutchConverterControl,
            50 => GlobalFunctionEnum::AuxiliaryHeater,
            51 => GlobalFunctionEnum::ForwardLookingCollisionWarningSystem,
            52 => GlobalFunctionEnum::ChassisController,
            53 => GlobalFunctionEnum::AlternatorChargingSystem,
            54 => GlobalFunctionEnum::CommunicationsUnitCellular,
            55 => GlobalFunctionEnum::CommunicationsUnitSatellite,
            56 => GlobalFunctionEnum::CommunicationsUnitRadio,
            57 => GlobalFunctionEnum::SteeringColumnUnit,
            58 => GlobalFunctionEnum::FanDriveControl,
            59 => GlobalFunctionEnum::Starter,
            60 => GlobalFunctionEnum::CabDisplay,
            61 => GlobalFunctionEnum::FileServerPrinter,
            62 => GlobalFunctionEnum::OnBoardDiagnosticUnit,
            63 => GlobalFunctionEnum::EngineValveController,
            64 => GlobalFunctionEnum::EnduranceBraking,
            65 => GlobalFunctionEnum::GasFlowMeasurement,
            66 => GlobalFunctionEnum::IOController,
            67 => GlobalFunctionEnum::ElectricalSystemController,
            68 => GlobalFunctionEnum::AfterTreatmentSystemGasMeasurement,
            69 => GlobalFunctionEnum::EngineEmissionAfterTreatmentSystem,
            70 => GlobalFunctionEnum::AuxiliaryRegenerationDevice,
            71 => GlobalFunctionEnum::TransferCaseControl,
            72 => GlobalFunctionEnum::CoolantValveController,
            73 => GlobalFunctionEnum::RolloverDetectionControl,
            74 => GlobalFunctionEnum::LubricationSystem,
            75 => GlobalFunctionEnum::SupplementalFan,
            76 => GlobalFunctionEnum::TemperatureSensor,
            77 => GlobalFunctionEnum::FuelPropertiesSensor,
            78 => GlobalFunctionEnum::FireSuppressionSystem,
            79 => GlobalFunctionEnum::PowerSystemsManager,
            80 => GlobalFunctionEnum::ElectricPowertrain,
            81 => GlobalFunctionEnum::HydraulicPowertrain,
            82 => GlobalFunctionEnum::FileServer,
            83 => GlobalFunctionEnum::Printer,
            84 => GlobalFunctionEnum::StartAidDevice,
            85 => GlobalFunctionEnum::EngineInjectionControlModule,
            86 => GlobalFunctionEnum::EVCommunicationController,
            87 => GlobalFunctionEnum::DriverImpairmentDevice,
            88 => GlobalFunctionEnum::ElectricPowerConverter,
            89 => GlobalFunctionEnum::SupplyEquipmentCommunicationController,
            90 => GlobalFunctionEnum::VehicleAdapterCommunicationController,
            128 => GlobalFunctionEnum::Reserved,
            129 => GlobalFunctionEnum::OffBoardDiagnosticServiceTool,
            130 => GlobalFunctionEnum::OnBoardDataLogger,
            131 => GlobalFunctionEnum::PCKeyboard,
            132 => GlobalFunctionEnum::SafetyRestraintSystem,
            133 => GlobalFunctionEnum::Turbocharger,
            134 => GlobalFunctionEnum::GroundBasedSpeedSensor,
            135 => GlobalFunctionEnum::Keypad,
            136 => GlobalFunctionEnum::HumiditySensor,
            137 => GlobalFunctionEnum::ThermalManagementSystemController,
            138 => GlobalFunctionEnum::BrakeStrokeAlert,
            139 => GlobalFunctionEnum::OnBoardAxleGroupScale,
            140 => GlobalFunctionEnum::OnBoardAxleGroupDisplay,
            141 => GlobalFunctionEnum::BatteryCharger,
            142 => GlobalFunctionEnum::TurbochargerCompressorBypass,
            143 => GlobalFunctionEnum::TurbochargerWastegate,
            144 => GlobalFunctionEnum::Throttle,
            145 => GlobalFunctionEnum::InertialSensor,
            146 => GlobalFunctionEnum::FuelActuator,
            147 => GlobalFunctionEnum::EngineExhaustGasRecirculation,
            148 => GlobalFunctionEnum::EngineExhaustBackpressure,
            149 => GlobalFunctionEnum::OnBoardBinWeighingScale,
            150 => GlobalFunctionEnum::OnBoardBinWeighingScaleDisplay,
            151 => GlobalFunctionEnum::EngineCylinderPressureMonitoringSystem,
            152 => GlobalFunctionEnum::ObjectDetection,
            153 => GlobalFunctionEnum::ObjectDetectionDisplay,
            154 => GlobalFunctionEnum::ObjectDetectionSensor,
            155 => GlobalFunctionEnum::PersonnelDetectionDevice,
            255 => GlobalFunctionEnum::NotAvailable,
            _ => GlobalFunctions::default(),
        }
    }
}

impl From<GlobalFunctions> for u8 {
    fn from(value: GlobalFunctions) -> Self {
        value as u8
    }
}


/// Struct containing all On Highway Equipment Function ID's
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum OnHighwayEquipmentFunctionEnum {
    Tachograph = 128,
    DoorController = 129,
    ArticulationTurntableControl = 130,
    BodyToVehicleInterfaceControl = 131,
    SlopeSensor = 132,
    RetarderDisplay = 134,
    DifferentialLockController = 135,
    LowVoltageDisconnect = 136,
    RoadwayInformation = 137,
    AutomatedDriving = 138,
    #[default] NotAvailable = 255,
}

impl core::fmt::Display for OnHighwayEquipmentFunctionEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for OnHighwayEquipmentFunctionEnum {
    fn from(value: u8) -> Self {
        match value {
            128 => OnHighwayEquipmentFunctionEnum::Tachograph,
            129 => OnHighwayEquipmentFunctionEnum::DoorController,
            130 => OnHighwayEquipmentFunctionEnum::ArticulationTurntableControl,
            131 => OnHighwayEquipmentFunctionEnum::BodyToVehicleInterfaceControl,
            132 => OnHighwayEquipmentFunctionEnum::SlopeSensor,
            134 => OnHighwayEquipmentFunctionEnum::RetarderDisplay,
            135 => OnHighwayEquipmentFunctionEnum::DifferentialLockController,
            136 => OnHighwayEquipmentFunctionEnum::LowVoltageDisconnect,
            137 => OnHighwayEquipmentFunctionEnum::RoadwayInformation,
            138 => OnHighwayEquipmentFunctionEnum::AutomatedDriving,
            255 => OnHighwayEquipmentFunctionEnum::NotAvailable,
            _ => OnHighwayEquipmentFunctionEnum::default(),
        }
    }
}

impl From<OnHighwayEquipmentFunctionEnum> for u8 {
    fn from(value: OnHighwayEquipmentFunctionEnum) -> Self {
        value as u8
    }
}







// 128
// 129
// 130
// 131
// 132
// 133
// 255
// 255
// 255





// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 139
// 140
// 141
// 142
// 255



// 129
// 130
// 131
// 132
// 134
// 255



// 132
// 135
// 136
// 255




// 132
// 135
// 136
// 255



// 128
// 129
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 255




// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 255
// 132
// 133
// 134
// 135
// 255
// 128
// 129
// 131
// 132
// 133
// 135
// 255
// 255
// 132
// 136
// 255
// 255
// 132
// 255
// 132
// 255
// 128
// 132
// 133
// 134
// 135
// 255
// 132
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 132
// 132
// 132
// 132
// 132
// 132
// 128
// 129
// 130
// 132
// 133
// 134
// 135
// 128
// 129
// 130
// 132
// 133
// 134
// 135
// 132
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 139
// 140
// 141
// 142
// 143
// 144
// 145
// 146
// 255
// 128
// 255
// 255
// 255
// 128
// 255
// 128
// 255
// 255
// 255
// 128
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 128
// 129
// 130
// 255
// 255
// 255
// 130
// 130
// 140
// 130
// 140
// 150
// 160
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 200
// 210
// 220
// 130
// 140
// 145
// 150
// 155
// 160
// 170
// 200
// 205
// 210
// 220
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 200
// 255
// 255
// 255
// 128
// 129
// 130
// 131
// 132
// 255
// 255
