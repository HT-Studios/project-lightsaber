#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PeripheralComponentInterconnectDeviceType {
    UnknownType,

    // Base Class 0x00: Unclassified
    Unclassified(UnclassifiedPeripheralComponentInterconnectDevice),

    // Base Class 0x01: Mass Storage Controller
    MassStorageController(MassStorageControllerPeripheralComponentInterconnectDevice),

    // Base Class 0x02: Network Controller
    NetworkController(NetworkControllerPeripheralComponentInterconnectDevice),

    // Base Class 0x03: Display Controller
    DisplayController(DisplayControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x04: Multimedia Controller
    MultimediaController(MultimediaControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x05: Memory Controller
    MemoryController(MemoryControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x06: Bridge Device
    BridgeDevice(BridgeDevicePeripheralControllerInterconnectDevice),

    // Base Class 0x07: Simple Communication Controller
    SimpleCommunicationController(SimpleCommunicationPeripheralControllerInterconnectDevice),

    // Base Class 0x08: Base System Peripheral
    BaseSystemPeripheral(BaseSystemPeripheralPeripheralControllerInterconnectDevice),

    // Base Class 0x09: Input Device Controller
    InputDeviceController(InputDeviceControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x0A: Docking Station
    DockingStation(DockingStationPeripheralControllerInterconnectDevice),

    // Base Class 0x0B: Processor
    Processor(ProcessorPeripheralControllerInterconnectDevice),

    // Base Class 0x0C: Serial Bus Controller
    SerialBusController(SerialBusControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x0D: Wireless Controller
    WirelessController(WirelessControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x0E: Intelligent Controller
    IntelligentController(IntelligentControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x0F: Satellite Controller
    SatelliteController(SatelliteControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x10: Encryption Controller
    EncryptionController(EncryptionControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x11: Signal Processing Controller
    SignalProcessingController(SignalProcessingControllerPeripheralControllerInterconnectDevice),

    // Base Class 0x12: Processing Accelerator
    ProcessingAccelerator,

    // Base Class 0x13: Non-Essential Instrumentation
    NonEssentialIntrumentation,

    // Base Class 0x40: Co-Processor
    Coprocessor,

    // Base Class 0xFF
    UnassignedClass
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnclassifiedPeripheralComponentInterconnectDevice {
    // Subclass 0x00
    NonVideoGraphicsArrayCompatibleDevice,
    // Subclass 0x01
    VideoGraphicsArrayCompatibleDevice
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MassStorageControllerPeripheralComponentInterconnectDevice {
    // Subclass 0x00
    SmallComputerSystemInterfaceBusController,
    // Base Class 0x01; Subclass 0x01
    IntegratedDriveElectronicsController(IntegratedDriveElectronicsControllerProgrammingInterfaceByte),
    // Subclass 0x02
    FloppyDiskController,
    // Subclass 0x03
    IntelligentPeripheralInterfaceBusController,
    // Subclass 0x04
    RedundantArrayOfIndependentDisksController,
    // Subclass 0x05
    AdvancedTechnologyAttachmentController(AdvancedTechnologyAttachmentControllerProgrammingInterfaceByte),
    // Subclass 0x06
    SerialAdvancedTechnologyAttachment(SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte),
    // Subclass 0x07
    SerialAttachedSmallComputerSystemInterface(SerialAttachedSmallComputerSystemInterfaceProgrammingInterfaceByte),
    // Subclass 0x08
    NonVolatileMemoryController(NonVolatileMemoryControllerProgrammingInterfaceByte),
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkControllerPeripheralComponentInterconnectDevice {
    // Subclass 0x00
    EthernetController,
    // Subclass 0x01
    TokenRingController,
    // Subclass 0x02
    FiberDistributedDataInterfaceController,
    // Subclass 0x03,
    AsynchronousTransferModeController,
    // Subclass 0x04
    IntegratedServicesDigitalNetworkController,
    // Subclass 0x05
    WorldFipController,
    // Subclass 0x06
    PeripheralComponentInterconnectIndustrialComputerManufacturersGroupSpecification2_14MultiComputing,
    // Subclass 0x07
    InfinibandController,
    // Subclass 0x08
    FabricController,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisplayControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    VideoGraphicsArrayCompatibleController(VideoGraphicsArrayCompatibleControllerProgrammingInterfaceByte),
    // Subclass 0x01
    ExtendedGraphicsArrayController,
    // Subclass 0x02
    NotVideoGraphicsArrayCompatible3DimensionalController,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MultimediaControllerPeripheralControllerInterconnectDevice {
    // Prog IF 0x00
    MultimediaVideoController,
    // Prog IF 0x01
    MultiMediaAudioController,
    // Prog IF 0x02
    ComputerTelephonyDevice,
    // Prog IF 0x03
    AudioDevice,
    // Prog IF 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    RandomAccessMemoryController,
    // Subclass 0x01
    FlashController,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BridgeDevicePeripheralControllerInterconnectDevice {
    // Subclass 0x00
    HostBridge,
    // Subclass 0x01
    IndustryStandardArchitectureBridge,
    // Subclass 0x02
    ExtendedIndustryStandardArchitectureBridge,
    // Subclass 0x03
    MicroChannelArchitectureBridge,
    // Subclass 0x04
    PeripheralControllerInterconnectBridge(PeripheralControllerInterconnectBridgeProgrammingInterfaceByte),
    // Subclass 0x05
    PersonalComputerMemoryCardInternationalAssociationCardBridge,
    // Subclss 0x06
    NuBusBridge,
    // Subclass 0x07
    CardBusBridge,
    // Subclass 0x08
    RACEwayBridge(RACEwayBridgeProgrammingInterfaceByte),
    // Subclass 0x09
    SemitransparentPeripheralControllerInterfaceToPeripheralControllerInterfaceBridge(SemitransparentPeriperalControllerInterfaceToPeripheralControllerInterfaceBridgeProgrammingInterfaceByte),
    // Subclass 0x0A
    InfinibandToPeripheralControllerInterfaceBridge,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleCommunicationPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    SerialController(SerialControllerProgrammingInterfaceByte),
    // Subclass 0x01
    ParallelController(ParallelControllerProgrammingInterfaceByte),
    // Subclass 0x02
    MultiportSerialController,
    // Subclass 0x03
    Modem(ModemProgrammingInterfaceByte),
    // Subclass 0x04
    InstituteOfElectricalAndElectronicsEngineersGeneralPurposeInterfaceBusController,
    // Subclass 0x05
    SmartCard,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaseSystemPeripheralPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    PeripheralInterfaceController(PeripheralInterfaceControllerProgrammingInterfaceByte),
    // Subclass 0x01
    DirectMemoryAccessController(DirectMemoryAccessControllerProgrammingInterfaceByte),
    // Subclass 0x02
    Timer(TimerProgrammingInterfaceByte),
    // Subclass 0x03
    RemoteTimeClock(RemoteTimeClockProgrammingInterfaceByte),
    // Subclass 0x04
    PeripheralControllerInterconnectHotPlugController,
    // Subclass 0x05
    SecureDigitalHostController,
    // Subclass 0x06
    InputOutputMemoryManagementUnit,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputDeviceControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    KeyboardController,
    // Subclass 0x01
    DigitizerPen,
    // Subclass 0x02
    MouseController,
    // Subclass 0x03
    ScannerController,
    // Subclass 0x04
    GameportController(GameportControllerProgrammingInterfaceByte),
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DockingStationPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    Generic,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessorPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    Intel386,
    // Subclass 0x01
    Intel486,
    // Subclass 0x02
    IntelPentinum,
    // Subclass 0x03
    IntelPentinumPro,
    // Subclass 0x10
    Alpha,
    // Subclass 0x20
    PowerPC,
    // Subclass 0x30
    MIPS,
    // Subclass 0x40
    Coprocessor,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialBusControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    FirewireController(FirewireControllerProgrammingInterfaceByte),
    // Subclass 0x01
    ACCESSBus,
    // Subclass 0x02
    SerialStorageArchitecture,
    // Subclass 0x03
    UniversalSerialBusController(UniversalSerialBusControllerProgrammingInterfaceByte),
    // Subclass 0x04
    FibreChannel,
    // Subclass 0x05
    SystemManagementBus,
    // Subclass 0x06
    Infiniband,
    // Subclass 0x07
    IntelligentPlatformManagementInterface(IntelligentPlatformManagementInterfaceProgrammingInterfaceByte),
    // Subclass 0x08
    SerialRealTimeCommunicationSystemInterface,
    // Subclass 0x09
    ControllerAreaNetwork,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WirelessControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    InfraredDataAssociationCompatibleController,
    // Subclass 0x01
    ConsumerInfraredController,
    // Subclass 0x10
    RadioFrequencyController,
    // Subclass 0x11
    BluetoothController,
    // Subclass 0x12
    BroadbandController,
    // Subclass 0x20
    EthernetController802_1a,
    // Subclass 0x21
    EthernetController802_1b,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntelligentControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    I20
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SatelliteControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x01
    SatelliteTelevisionController,
    // Subclass 0x02
    SatelliteAudioController,
    // Subclass 0x03
    SatelliteVoiceController,
    // Subclass 0x04
    SatelliteDataController
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EncryptionControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    NetworkAndComputingEncryptionOrDecrption,
    // Subclass 0x01
    EntertainmentEncryptionOrDecryption,
    // Subclass 0x80
    OtherEncryptionOrDecryption
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignalProcessingControllerPeripheralControllerInterconnectDevice {
    // Subclass 0x00
    DataPathIOModules,
    // Subclass 0x01
    PerformanceCounters,
    // Subclass 0x10
    CommunicationSynchronizer,
    // Subclass 0x20
    SignalProcessingManagement,
    // Subclass 0x80
    Other
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdvancedTechnologyAttachmentControllerProgrammingInterfaceByte {
    // Prog IF 0x20
    SingleDirectMemoryAccess,
    // Prog IF 0x30
    ChainedDirectMemoryAccess
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectMemoryAccessControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    Generic8237Compatible,
    // Prog IF 0x01
    IndustryStandardArchitectureCompatible,
    // Prog IF 0x02
    ExtendedIndustryStandardArchitectureCompatible
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FirewireControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    Generic,
    // Prog IF 0x01
    OpenHostControllerInterface
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameportControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    Generic,
    // Prog IF 0x10
    Extended
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegratedDriveElectronicsControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    IndustryStandardArchitectureCompatibilityModeOnlyController,
    // Prog IF 0x05
    PeripheralComponentInterconnectNativeModeOnlyController,
    // Prog IF 0x0A
    IndustryStandardArchitectureCompatibilityModeControllerBothChannelsSwitchedToPeripheralComponentInterconnectNativeModeSupported,
    // Prog IF 0x0F
    PeripheralComponentInterconnectNativeModeControllerBothChannelsSwitchedToIndustryStandardArchitectureCompatibilityModeSupported,
    // Prog IF 0x80
    IndustryStandardArchitectureCompatibilityModeOnlyControllerBusMasteringSupported,
    // Prog IF 0x85
    PeripheralComponentInterconnectNativeModeOnlyControllerBusMasteringSupported,
    // Prog IF 0x8A
    IndustryStandardArchitectureCompatibilityModeControllerBothChannelsSwitchedToPeripheralComponentInterconnectNativeModeSupportedBusMasteringSupported,
    // Prog IF 0x8F
    PeripheralComponentInterconnectNativeModeControllerBothChannelsSwitchedToIndustryStandardArchitectureCompatibilityModeSupportedBusMasteringSupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntelligentPlatformManagementInterfaceProgrammingInterfaceByte {
    // Prog IF 0x00
    SemiconductorManufacturingInternationalCorporation,
    // Prog IF 0x01
    KeyboardControllerStyle,
    // Prog IF 0x02
    BlockTransfer
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModemProgrammingInterfaceByte {
    // Prog IF 0x00
    GenericModem,
    // Prog IF 0x01
    Hayes16450CompatibleInterface,
    // Prog IF 0x02
    Hayes16550CompatibleInterface,
    // Prog IF 0x03
    Hayes16650CompatibleInterface,
    // Prog IF 0x04
    Hayes16750CompatibleInterface
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NonVolatileMemoryControllerProgrammingInterfaceByte {
    // Prog IF 0x01
    NonVolatileMemoryHostControllerInterface,
    // Prog IF 0x02
    NonVolatileMemoryExpress
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParallelControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    StandardParallelPort,
    // Prog IF 0x01
    BidirectionalParallelPort,
    // Prog IF 0x02
    EnhancedCompatibilityPort1XCompilantParallelPort,
    // Prog IF 0x03
    InstituteOfElectricalAndElectronicsEngineers1284Controller,
    // Prog IF 0xFE
    InstituteOfElectricalAndElectronicsEngineers1284TargetDevice
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PeripheralControllerInterconnectBridgeProgrammingInterfaceByte {
    // Prog IF 0x00
    NormalDecode,
    // Prog IF 0x01
    SubtractiveDecode
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PeripheralInterfaceControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    Generic8259Compatible,
    // Prog IF 0x01
    IndustryStandardArchitectureCompatible,
    // Prog IF 0x02
    ExtendedIndustryStandardArchitectureCompatible,
    // Prog IF 0x03
    IoAdvancedProgrammableInterfaceControllerInterruptController,
    // Prog IF 0x04
    IoXAdvancedProgrammableInterfaceControllerInterruptController
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RACEwayBridgeProgrammingInterfaceByte {
    // Prog IF 0x00
    TransparentMode,
    // Prog IF 0x01
    EndpointMode
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemoteTimeClockProgrammingInterfaceByte {
    // Prog IF 0x00
    GenericRemoteTimeClock,
    // Prog IF 0x01
    IndustryStandardArchitectureCompatible
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte {
    // Prog IF 0x00
    VendorSpecificInterface,
    // Prog IF 0x01
    AdvancedHostControllerInterfaceVersion1_0,
    // Prog IF 0x02
    SerialStorageBus
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    UniversalAsynchronousReceiverTransmitter8250Compatible,
    // Prog IF 0x01
    UniversalAsynchronousReceiverTransmitter16450Compatible,
    // Prog IF 0x02
    UniversalAsynchronousReceiverTransmitter16550Compatible,
    // Prog IF 0x03
    UniversalAsynchronousReceiverTransmitter16650Compatible,
    // Prog IF 0x04
    UniversalAsynchronousReceiverTransmitter16750Compatible,
    // Prog IF 0x05
    UniversalAsynchronousReceiverTransmitter16850Compatible,
    // Prog IF 0x06
    UniversalAsynchronousReceiverTransmitter16950Compatible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemitransparentPeriperalControllerInterfaceToPeripheralControllerInterfaceBridgeProgrammingInterfaceByte {
    // Prog IF 0x40
    PrimaryBusTowardsHostProcessor,
    // Prog IF 0x80
    SecondaryBusTowardsHostProcessor
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialAttachedSmallComputerSystemInterfaceProgrammingInterfaceByte {
    // Prog IF 0x00
    SerialAttachedSmallComputerSystemInterface,
    // Prog IF 0x01
    SerialStorageBus
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimerProgrammingInterfaceByte {
    // Prog IF 0x00
    Generic8254Compatible,
    // Prog IF 0x01
    IndustryStandardArchitectureCompatible,
    // Prog IF 0x02
    ExtendedIndustryStandardArchitectureCompatible,
    // Prog IF 0x03
    HighPrecisionEventTimer
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UniversalSerialBusControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    UniversalHostControllerInterfaceController,
    // Prog IF 0x01
    OpenHostControllerInterfaceController,
    // Prog IF 0x02
    UniversalSerialBus2Controller,
    // Prog IF 0x03
    UniversalSerialBus3Controller,
    // Prog IF 0x80
    Unspecified,
    // Prog IF 0xFE
    UniversalSerialBusDevice
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VideoGraphicsArrayCompatibleControllerProgrammingInterfaceByte {
    // Prog IF 0x00
    VideoGraphicsArrayController,
    // Prog IF 0x01
    InternationalBusinessMachinesCorporation8514CompatibleController
}
