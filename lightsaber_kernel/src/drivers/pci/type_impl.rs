use crate::drivers::pci::r#type::{
    PeripheralComponentInterconnectDeviceType,

    BaseSystemPeripheralPeripheralControllerInterconnectDevice,
    BridgeDevicePeripheralControllerInterconnectDevice,
    DisplayControllerPeripheralControllerInterconnectDevice,
    DockingStationPeripheralControllerInterconnectDevice,
    EncryptionControllerPeripheralControllerInterconnectDevice,
    InputDeviceControllerPeripheralControllerInterconnectDevice,
    IntelligentControllerPeripheralControllerInterconnectDevice,
    MassStorageControllerPeripheralComponentInterconnectDevice,
    MemoryControllerPeripheralControllerInterconnectDevice,
    MultimediaControllerPeripheralControllerInterconnectDevice,
    NetworkControllerPeripheralComponentInterconnectDevice,
    ProcessorPeripheralControllerInterconnectDevice,
    SatelliteControllerPeripheralControllerInterconnectDevice,
    SerialBusControllerPeripheralControllerInterconnectDevice,
    SignalProcessingControllerPeripheralControllerInterconnectDevice,
    SimpleCommunicationPeripheralControllerInterconnectDevice,
    UnclassifiedPeripheralComponentInterconnectDevice,
    WirelessControllerPeripheralControllerInterconnectDevice,

    AdvancedTechnologyAttachmentControllerProgrammingInterfaceByte,
    DirectMemoryAccessControllerProgrammingInterfaceByte,
    FirewireControllerProgrammingInterfaceByte,
    GameportControllerProgrammingInterfaceByte,
    IntegratedDriveElectronicsControllerProgrammingInterfaceByte,
    IntelligentPlatformManagementInterfaceProgrammingInterfaceByte,
    ModemProgrammingInterfaceByte,
    NonVolatileMemoryControllerProgrammingInterfaceByte,
    ParallelControllerProgrammingInterfaceByte,
    PeripheralControllerInterconnectBridgeProgrammingInterfaceByte,
    PeripheralInterfaceControllerProgrammingInterfaceByte,
    RACEwayBridgeProgrammingInterfaceByte,
    RemoteTimeClockProgrammingInterfaceByte,
    SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte,
    SerialControllerProgrammingInterfaceByte,
    SemitransparentPeriperalControllerInterfaceToPeripheralControllerInterfaceBridgeProgrammingInterfaceByte,
    SerialAttachedSmallComputerSystemInterfaceProgrammingInterfaceByte,
    TimerProgrammingInterfaceByte,
    UniversalSerialBusControllerProgrammingInterfaceByte,
    VideoGraphicsArrayCompatibleControllerProgrammingInterfaceByte
};

impl PeripheralComponentInterconnectDeviceType {
    pub fn new(base_class: u32, subclass: Option<u32>, prog_if: Option<u32>) -> Self {
        match (base_class, subclass, prog_if) {
            (0x00, Some(0x00), None) => Self::Unclassified(
                UnclassifiedPeripheralComponentInterconnectDevice::NonVideoGraphicsArrayCompatibleDevice
            ),
            (0x00, Some(0x01), None) => Self::Unclassified(
                UnclassifiedPeripheralComponentInterconnectDevice::VideoGraphicsArrayCompatibleDevice
            ),

            (0x01, Some(0x00), None) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SmallComputerSystemInterfaceBusController
            ),
            (0x01, Some(0x01), Some(0x00)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatibilityModeOnlyController
                )
            ),
            (0x01, Some(0x01), Some(0x05)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::PeripheralComponentInterconnectNativeModeOnlyController
                )
            ),
            (0x01, Some(0x01), Some(0x0A)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatibilityModeControllerBothChannelsSwitchedToPeripheralComponentInterconnectNativeModeSupported
                )
            ),
            (0x01, Some(0x01), Some(0x0F)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::PeripheralComponentInterconnectNativeModeControllerBothChannelsSwitchedToIndustryStandardArchitectureCompatibilityModeSupported
                )
            ),
            (0x01, Some(0x01), Some(0x80)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatibilityModeOnlyControllerBusMasteringSupported
                )
            ),
            (0x01, Some(0x01), Some(0x85)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::PeripheralComponentInterconnectNativeModeOnlyControllerBusMasteringSupported
                )
            ),
            (0x01, Some(0x01), Some(0x8A)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatibilityModeControllerBothChannelsSwitchedToPeripheralComponentInterconnectNativeModeSupportedBusMasteringSupported
                )
            ),
            (0x01, Some(0x01), Some(0x8F)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntegratedDriveElectronicsController(
                    IntegratedDriveElectronicsControllerProgrammingInterfaceByte::PeripheralComponentInterconnectNativeModeControllerBothChannelsSwitchedToIndustryStandardArchitectureCompatibilityModeSupportedBusMasteringSupported
                )
            ),
            (0x01, Some(0x02), None) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::FloppyDiskController
            ),
            (0x01, Some(0x03), None) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::IntelligentPeripheralInterfaceBusController
            ),
            (0x01, Some(0x04), None) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::RedundantArrayOfIndependentDisksController
            ),
            (0x01, Some(0x05), Some(0x20)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::AdvancedTechnologyAttachmentController(
                    AdvancedTechnologyAttachmentControllerProgrammingInterfaceByte::SingleDirectMemoryAccess
                )
            ),
            (0x01, Some(0x05), Some(0x30)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::AdvancedTechnologyAttachmentController(
                    AdvancedTechnologyAttachmentControllerProgrammingInterfaceByte::ChainedDirectMemoryAccess
                )
            ),
            (0x01, Some(0x06), Some(0x00)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SerialAdvancedTechnologyAttachment(
                    SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte::VendorSpecificInterface
                )
            ),
            (0x01, Some(0x06), Some(0x01)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SerialAdvancedTechnologyAttachment(
                    SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte::AdvancedHostControllerInterfaceVersion1_0
                )
            ),
            (0x01, Some(0x06), Some(0x02)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SerialAdvancedTechnologyAttachment(
                    SerialAdvancedTechnologyAttachmentProgrammingInterfaceByte::SerialStorageBus
                )
            ),
            (0x01, Some(0x07), Some(0x00)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SerialAttachedSmallComputerSystemInterface(
                    SerialAttachedSmallComputerSystemInterfaceProgrammingInterfaceByte::SerialAttachedSmallComputerSystemInterface
                )
            ),
            (0x01, Some(0x07), Some(0x01)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::SerialAttachedSmallComputerSystemInterface(
                    SerialAttachedSmallComputerSystemInterfaceProgrammingInterfaceByte::SerialStorageBus
                )
            ),
            (0x01, Some(0x08), Some(0x01)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::NonVolatileMemoryController(
                    NonVolatileMemoryControllerProgrammingInterfaceByte::NonVolatileMemoryHostControllerInterface
                )
            ),
            (0x01, Some(0x08), Some(0x02)) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::NonVolatileMemoryController(
                    NonVolatileMemoryControllerProgrammingInterfaceByte::NonVolatileMemoryExpress
                )
            ),
            (0x01, Some(0x80), None) => Self::MassStorageController(
                MassStorageControllerPeripheralComponentInterconnectDevice::Other
            ),

            (0x02, Some(0x00), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::EthernetController
            ),
            (0x02, Some(0x01), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::TokenRingController
            ),
            (0x02, Some(0x02), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::FiberDistributedDataInterfaceController
            ),
            (0x02, Some(0x03), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::AsynchronousTransferModeController
            ),
            (0x02, Some(0x04), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::IntegratedServicesDigitalNetworkController
            ),
            (0x02, Some(0x05), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::WorldFipController
            ),
            (0x02, Some(0x06), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::PeripheralComponentInterconnectIndustrialComputerManufacturersGroupSpecification2_14MultiComputing
            ),
            (0x02, Some(0x07), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::InfinibandController
            ),
            (0x02, Some(0x08), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::FabricController
            ),
            (0x02, Some(0x80), None) => Self::NetworkController(
                NetworkControllerPeripheralComponentInterconnectDevice::Other
            ),

            (0x03, Some(0x00), Some(0x00)) => Self::DisplayController(
                DisplayControllerPeripheralControllerInterconnectDevice::VideoGraphicsArrayCompatibleController(
                    VideoGraphicsArrayCompatibleControllerProgrammingInterfaceByte::VideoGraphicsArrayController
                )
            ),
            (0x03, Some(0x00), Some(0x01)) => Self::DisplayController(
                DisplayControllerPeripheralControllerInterconnectDevice::VideoGraphicsArrayCompatibleController(
                    VideoGraphicsArrayCompatibleControllerProgrammingInterfaceByte::InternationalBusinessMachinesCorporation8514CompatibleController
                )
            ),
            (0x03, Some(0x01), None) => Self::DisplayController(
                DisplayControllerPeripheralControllerInterconnectDevice::ExtendedGraphicsArrayController
            ),
            (0x03, Some(0x02), None) => Self::DisplayController(
                DisplayControllerPeripheralControllerInterconnectDevice::NotVideoGraphicsArrayCompatible3DimensionalController
            ),
            (0x03, Some(0x03), None) => Self::DisplayController(
                DisplayControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x04, Some(0x00), None) => Self::MultimediaController(
                MultimediaControllerPeripheralControllerInterconnectDevice::MultimediaVideoController
            ),
            (0x04, Some(0x01), None) => Self::MultimediaController(
                MultimediaControllerPeripheralControllerInterconnectDevice::MultiMediaAudioController
            ),
            (0x04, Some(0x02), None) => Self::MultimediaController(
                MultimediaControllerPeripheralControllerInterconnectDevice::ComputerTelephonyDevice
            ),
            (0x04, Some(0x03), None) => Self::MultimediaController(
                MultimediaControllerPeripheralControllerInterconnectDevice::AudioDevice
            ),
            (0x04, Some(0x80), None) => Self::MultimediaController(
                MultimediaControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x05, Some(0x00), None) => Self::MemoryController(
                MemoryControllerPeripheralControllerInterconnectDevice::RandomAccessMemoryController
            ),
            (0x05, Some(0x01), None) => Self::MemoryController(
                MemoryControllerPeripheralControllerInterconnectDevice::FlashController
            ),
            (0x05, Some(0x80), None) => Self::MemoryController(
                MemoryControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x06, Some(0x00), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::HostBridge
            ),
            (0x06, Some(0x01), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::IndustryStandardArchitectureBridge
            ),
            (0x06, Some(0x02), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::ExtendedIndustryStandardArchitectureBridge
            ),
            (0x06, Some(0x03), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::MicroChannelArchitectureBridge
            ),
            (0x06, Some(0x04), Some(0x00)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::PeripheralControllerInterconnectBridge(
                    PeripheralControllerInterconnectBridgeProgrammingInterfaceByte::NormalDecode
                )
            ),
            (0x06, Some(0x04), Some(0x01)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::PeripheralControllerInterconnectBridge(
                    PeripheralControllerInterconnectBridgeProgrammingInterfaceByte::SubtractiveDecode
                )
            ),
            (0x06, Some(0x05), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::PersonalComputerMemoryCardInternationalAssociationCardBridge
            ),
            (0x06, Some(0x06), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::NuBusBridge
            ),
            (0x06, Some(0x07), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::CardBusBridge
            ),
            (0x06, Some(0x08), Some(0x00)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::RACEwayBridge(
                    RACEwayBridgeProgrammingInterfaceByte::TransparentMode
                )
            ),
            (0x06, Some(0x08), Some(0x01)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::RACEwayBridge(
                    RACEwayBridgeProgrammingInterfaceByte::EndpointMode
                )
            ),
            (0x06, Some(0x09), Some(0x40)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::SemitransparentPeripheralControllerInterfaceToPeripheralControllerInterfaceBridge(
                    SemitransparentPeriperalControllerInterfaceToPeripheralControllerInterfaceBridgeProgrammingInterfaceByte::PrimaryBusTowardsHostProcessor
                )
            ),
            (0x06, Some(0x09), Some(0x80)) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::SemitransparentPeripheralControllerInterfaceToPeripheralControllerInterfaceBridge(
                    SemitransparentPeriperalControllerInterfaceToPeripheralControllerInterfaceBridgeProgrammingInterfaceByte::SecondaryBusTowardsHostProcessor
                )
            ),
            (0x06, Some(0x0A), None) => Self::BridgeDevice(
                BridgeDevicePeripheralControllerInterconnectDevice::InfinibandToPeripheralControllerInterfaceBridge
            ),

            (0x07, Some(0x00), Some(0x00)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter8250Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x01)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16450Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x02)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16550Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x03)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16650Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x04)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16750Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x05)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16850Compatible
                )
            ),
            (0x07, Some(0x00), Some(0x06)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SerialController(
                    SerialControllerProgrammingInterfaceByte::UniversalAsynchronousReceiverTransmitter16950Compatible
                )
            ),
            (0x07, Some(0x01), Some(0x00)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::ParallelController(
                    ParallelControllerProgrammingInterfaceByte::StandardParallelPort
                )
            ),
            (0x07, Some(0x01), Some(0x01)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::ParallelController(
                    ParallelControllerProgrammingInterfaceByte::BidirectionalParallelPort
                )
            ),
            (0x07, Some(0x01), Some(0x02)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::ParallelController(
                    ParallelControllerProgrammingInterfaceByte::EnhancedCompatibilityPort1XCompilantParallelPort
                )
            ),
            (0x07, Some(0x01), Some(0x03)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::ParallelController(
                    ParallelControllerProgrammingInterfaceByte::InstituteOfElectricalAndElectronicsEngineers1284Controller
                )
            ),
            (0x07, Some(0x01), Some(0xFE)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::ParallelController(
                    ParallelControllerProgrammingInterfaceByte::InstituteOfElectricalAndElectronicsEngineers1284TargetDevice
                )
            ),
            (0x07, Some(0x02), None) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::MultiportSerialController
            ),
            (0x07, Some(0x03), Some(0x00)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Modem(
                    ModemProgrammingInterfaceByte::GenericModem
                )
            ),
            (0x07, Some(0x03), Some(0x01)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Modem(
                    ModemProgrammingInterfaceByte::Hayes16450CompatibleInterface
                )
            ),
            (0x07, Some(0x03), Some(0x02)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Modem(
                    ModemProgrammingInterfaceByte::Hayes16550CompatibleInterface
                )
            ),
            (0x07, Some(0x03), Some(0x03)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Modem(
                    ModemProgrammingInterfaceByte::Hayes16650CompatibleInterface
                )
            ),
            (0x07, Some(0x03), Some(0x04)) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Modem(
                    ModemProgrammingInterfaceByte::Hayes16750CompatibleInterface
                )
            ),
            (0x07, Some(0x04), None) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::InstituteOfElectricalAndElectronicsEngineersGeneralPurposeInterfaceBusController
            ),
            (0x07, Some(0x05), None) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::SmartCard
            ),
            (0x07, Some(0x80), None) => Self::SimpleCommunicationController(
                SimpleCommunicationPeripheralControllerInterconnectDevice::Other
            ),

            (0x08, Some(0x00), Some(0x00)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralInterfaceController(
                    PeripheralInterfaceControllerProgrammingInterfaceByte::Generic8259Compatible
                )
            ),
            (0x08, Some(0x00), Some(0x01)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralInterfaceController(
                    PeripheralInterfaceControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x00), Some(0x02)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralInterfaceController(
                    PeripheralInterfaceControllerProgrammingInterfaceByte::ExtendedIndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x00), Some(0x03)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralInterfaceController(
                    PeripheralInterfaceControllerProgrammingInterfaceByte::IoAdvancedProgrammableInterfaceControllerInterruptController
                )
            ),
            (0x08, Some(0x00), Some(0x04)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralInterfaceController(
                    PeripheralInterfaceControllerProgrammingInterfaceByte::IoXAdvancedProgrammableInterfaceControllerInterruptController
                )
            ),
            (0x08, Some(0x01), Some(0x00)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::DirectMemoryAccessController(
                    DirectMemoryAccessControllerProgrammingInterfaceByte::Generic8237Compatible
                )
            ),
            (0x08, Some(0x01), Some(0x01)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::DirectMemoryAccessController(
                    DirectMemoryAccessControllerProgrammingInterfaceByte::IndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x01), Some(0x02)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::DirectMemoryAccessController(
                    DirectMemoryAccessControllerProgrammingInterfaceByte::ExtendedIndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x02), Some(0x00)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::Timer(
                    TimerProgrammingInterfaceByte::Generic8254Compatible
                )
            ),
            (0x08, Some(0x02), Some(0x01)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::Timer(
                    TimerProgrammingInterfaceByte::IndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x02), Some(0x02)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::Timer(
                    TimerProgrammingInterfaceByte::ExtendedIndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x02), Some(0x03)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::Timer(
                    TimerProgrammingInterfaceByte::HighPrecisionEventTimer
                )
            ),
            (0x08, Some(0x03), Some(0x00)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::RemoteTimeClock(
                    RemoteTimeClockProgrammingInterfaceByte::GenericRemoteTimeClock
                )
            ),
            (0x08, Some(0x03), Some(0x01)) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::RemoteTimeClock(
                    RemoteTimeClockProgrammingInterfaceByte::IndustryStandardArchitectureCompatible
                )
            ),
            (0x08, Some(0x04), None) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::PeripheralControllerInterconnectHotPlugController
            ),
            (0x08, Some(0x05), None) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::SecureDigitalHostController
            ),
            (0x08, Some(0x06), None) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::InputOutputMemoryManagementUnit
            ),
            (0x08, Some(0x80), None) => Self::BaseSystemPeripheral(
                BaseSystemPeripheralPeripheralControllerInterconnectDevice::Other
            ),

            (0x09, Some(0x00), None) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::KeyboardController
            ),
            (0x09, Some(0x01), None) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::DigitizerPen
            ),
            (0x09, Some(0x02), None) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::MouseController
            ),
            (0x09, Some(0x03), None) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::ScannerController
            ),
            (0x09, Some(0x04), Some(0x00)) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::GameportController(
                    GameportControllerProgrammingInterfaceByte::Generic
                )
            ),
            (0x09, Some(0x04), Some(0x10)) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::GameportController(
                    GameportControllerProgrammingInterfaceByte::Extended
                )
            ),
            (0x09, Some(0x80), None) => Self::InputDeviceController(
                InputDeviceControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x0A, Some(0x00), None) => Self::DockingStation(
                DockingStationPeripheralControllerInterconnectDevice::Generic
            ),
            (0x0A, Some(0x01), None) => Self::DockingStation(
                DockingStationPeripheralControllerInterconnectDevice::Other
            ),

            (0x0B, Some(0x00), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::Intel386
            ),
            (0x0B, Some(0x01), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::Intel486
            ),
            (0x0B, Some(0x02), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::IntelPentinum
            ),
            (0x0B, Some(0x03), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::IntelPentinumPro
            ),
            (0x0B, Some(0x10), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::Alpha
            ),
            (0x0B, Some(0x20), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::PowerPC
            ),
            (0x0B, Some(0x30), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::MIPS
            ),
            (0x0B, Some(0x40), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::Coprocessor
            ),
            (0x0B, Some(0x80), None) => Self::Processor(
                ProcessorPeripheralControllerInterconnectDevice::Other
            ),

            (0x0C, Some(0x00), Some(0x00)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::FirewireController(
                    FirewireControllerProgrammingInterfaceByte::Generic
                )
            ),
            (0x0C, Some(0x00), Some(0x10)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::FirewireController(
                    FirewireControllerProgrammingInterfaceByte::OpenHostControllerInterface
                )
            ),
            (0x0C, Some(0x01), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::ACCESSBus
            ),
            (0x0C, Some(0x02), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::SerialStorageArchitecture
            ),
            (0x0C, Some(0x03), Some(0x00)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::UniversalHostControllerInterfaceController
                )
            ),
            (0x0C, Some(0x03), Some(0x10)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::OpenHostControllerInterfaceController
                )
            ),
            (0x0C, Some(0x03), Some(0x20)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::UniversalSerialBus2Controller
                )
            ),
            (0x0C, Some(0x03), Some(0x30)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::UniversalSerialBus3Controller
                )
            ),
            (0x0C, Some(0x03), Some(0x80)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::Unspecified
                )
            ),
            (0x0C, Some(0x03), Some(0xFE)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::UniversalSerialBusController(
                    UniversalSerialBusControllerProgrammingInterfaceByte::UniversalSerialBusDevice
                )
            ),
            (0x0C, Some(0x04), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::FibreChannel
            ),
            (0x0C, Some(0x05), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::SystemManagementBus
            ),
            (0x0C, Some(0x06), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::Infiniband
            ),
            (0x0C, Some(0x07), Some(0x00)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::IntelligentPlatformManagementInterface(
                    IntelligentPlatformManagementInterfaceProgrammingInterfaceByte::SemiconductorManufacturingInternationalCorporation
                )
            ),
            (0x0C, Some(0x07), Some(0x01)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::IntelligentPlatformManagementInterface(
                    IntelligentPlatformManagementInterfaceProgrammingInterfaceByte::KeyboardControllerStyle
                )
            ),
            (0x0C, Some(0x07), Some(0x02)) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::IntelligentPlatformManagementInterface(
                    IntelligentPlatformManagementInterfaceProgrammingInterfaceByte::BlockTransfer
                )
            ),
            (0x0C, Some(0x08), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::SerialRealTimeCommunicationSystemInterface
            ),
            (0x0C, Some(0x09), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::ControllerAreaNetwork
            ),
            (0x0C, Some(0x80), None) => Self::SerialBusController(
                SerialBusControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x0D, Some(0x00), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::InfraredDataAssociationCompatibleController
            ),
            (0x0D, Some(0x01), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::ConsumerInfraredController
            ),
            (0x0D, Some(0x10), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::RadioFrequencyController
            ),
            (0x0D, Some(0x11), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::BluetoothController
            ),
            (0x0D, Some(0x12), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::BroadbandController
            ),
            (0x0D, Some(0x20), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::EthernetController802_1a
            ),
            (0x0D, Some(0x21), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::EthernetController802_1b
            ),
            (0x0D, Some(0x80), None) => Self::WirelessController(
                WirelessControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x0E, Some(0x00), None) => Self::IntelligentController(
                IntelligentControllerPeripheralControllerInterconnectDevice::I20
            ),

            (0x0F, Some(0x01), None) => Self::SatelliteController(
                SatelliteControllerPeripheralControllerInterconnectDevice::SatelliteTelevisionController
            ),
            (0x0F, Some(0x02), None) => Self::SatelliteController(
                SatelliteControllerPeripheralControllerInterconnectDevice::SatelliteAudioController
            ),
            (0x0F, Some(0x03), None) => Self::SatelliteController(
                SatelliteControllerPeripheralControllerInterconnectDevice::SatelliteVoiceController
            ),
            (0x0F, Some(0x04), None) => Self::SatelliteController(
                SatelliteControllerPeripheralControllerInterconnectDevice::SatelliteDataController
            ),

            (0x10, Some(0x00), None) => Self::EncryptionController(
                EncryptionControllerPeripheralControllerInterconnectDevice::NetworkAndComputingEncryptionOrDecrption
            ),
            (0x10, Some(0x10), None) => Self::EncryptionController(
                EncryptionControllerPeripheralControllerInterconnectDevice::EntertainmentEncryptionOrDecryption
            ),
            (0x10, Some(0x80), None) => Self::EncryptionController(
                EncryptionControllerPeripheralControllerInterconnectDevice::OtherEncryptionOrDecryption
            ),

            (0x11, Some(0x00), None) => Self::SignalProcessingController(
                SignalProcessingControllerPeripheralControllerInterconnectDevice::DataPathIOModules
            ),
            (0x11, Some(0x01), None) => Self::SignalProcessingController(
                SignalProcessingControllerPeripheralControllerInterconnectDevice::PerformanceCounters
            ),
            (0x11, Some(0x10), None) => Self::SignalProcessingController(
                SignalProcessingControllerPeripheralControllerInterconnectDevice::CommunicationSynchronizer
            ),
            (0x11, Some(0x20), None) => Self::SignalProcessingController(
                SignalProcessingControllerPeripheralControllerInterconnectDevice::SignalProcessingManagement
            ),
            (0x11, Some(0x80), None) => Self::SignalProcessingController(
                SignalProcessingControllerPeripheralControllerInterconnectDevice::Other
            ),

            (0x12, None, None) => Self::ProcessingAccelerator,

            (0x13, None, None) => Self::NonEssentialIntrumentation,

            (0x40, None, None) => Self::Coprocessor,

            (0xFF, None, None) => Self::UnassignedClass,

            _ => Self::UnknownType
        }
    }
}
