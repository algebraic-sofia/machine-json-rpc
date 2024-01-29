extern crate derive_builder;
extern crate serde;
extern crate serde_json;

use derive_builder::Builder;
use jsonrpsee::core::{params::ArrayParams, Error};
use serde::{Deserialize, Serialize};

pub type UnsignedInteger = u64;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct CLINTConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtimecmp: Option<UnsignedInteger>,
}

pub type StringDoaGddGA = String;
pub type BooleanVyG3AETh = bool;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct HTIFRuntimeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_console_putchar: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MemoryRangeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<UnsignedInteger>,
}
pub type FlashDriveConfigs = Vec<MemoryRangeConfig>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct HTIFConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_getchar: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromhost: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tohost: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_automatic: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_manual: Option<BooleanVyG3AETh>,
}
pub type FRegConfig = Vec<UnsignedInteger>;
pub type XRegConfig = Vec<UnsignedInteger>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ProcessorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f: Option<FRegConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcsr: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icycleinstret: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iflags: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ilrsc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marchid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcause: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcounteren: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcycle: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medeleg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menvcfg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mepc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mideleg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mie: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimpid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mip: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misa: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mscratch: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstatus: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtval: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtvec: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mvendorid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satp: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scause: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scounteren: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senvcfg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sscratch: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stval: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stvec: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XRegConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct RAMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    pub length: UnsignedInteger,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct RollupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_buffer: Option<MemoryRangeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_buffer: Option<MemoryRangeConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ROMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootargs: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct TLBConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchProcessorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XRegConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchRAMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<UnsignedInteger>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<UarchProcessorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<UarchRAMConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ConcurrencyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_merkle_tree: Option<UnsignedInteger>,
}
pub type Base64Hash = String;
pub type Base64HashArray = Vec<Base64Hash>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Proof {
    #[serde(rename = "log2_root_size")]
    pub log_2_root_size: UnsignedInteger,
    #[serde(rename = "log2_target_size")]
    pub log_2_target_size: UnsignedInteger,
    pub root_hash: Base64Hash,
    pub sibling_hashes: Base64HashArray,
    pub target_address: UnsignedInteger,
    pub target_hash: Base64Hash,
}
pub type Base64String = String;
pub type AccessType = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Access {
    pub address: UnsignedInteger,
    #[serde(rename = "log2_size")]
    pub log_2_size: UnsignedInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Proof>,
    pub read: Base64String,
    #[serde(rename = "type")]
    pub r#type: AccessType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<Base64String>,
}
pub type AccessArray = Vec<Access>;
pub type BracketType = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Bracket {
    pub text: StringDoaGddGA,
    #[serde(rename = "type")]
    pub r#type: BracketType,
    pub r#where: UnsignedInteger,
}
pub type BracketArray = Vec<Bracket>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct AccessLogType {
    pub has_annotations: BooleanVyG3AETh,
    pub has_proofs: BooleanVyG3AETh,
    pub has_large_data: BooleanVyG3AETh,
}
pub type NoteArray = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MachineConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clint: Option<CLINTConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_drive: Option<FlashDriveConfigs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htif: Option<HTIFConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<ProcessorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<RAMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollup: Option<RollupConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<ROMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlb: Option<TLBConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uarch: Option<UarchConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MachineRuntimeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<ConcurrencyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htif: Option<HTIFRuntimeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_root_hash_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_version_check: Option<bool>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct AccessLog {
    pub accesses: AccessArray,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brackets: Option<BracketArray>,
    pub log_type: AccessLogType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteArray>,
    pub has_large_data: bool
}
pub type CSR = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct SemanticVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<StringDoaGddGA>,
    pub major: UnsignedInteger,
    pub minor: UnsignedInteger,
    pub patch: UnsignedInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_release: Option<StringDoaGddGA>,
}
pub type InterpreterBreakReason = serde_json::Value;
pub type UarchInterpreterBreakReason = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOfMachineConfigMachineRuntimeConfigStringDoaGddGAMachineRuntimeConfigStringDoaGddGAUnsignedIntegerUnsignedIntegerAccessLogTypeBooleanVyG3AEThAccessLogMachineRuntimeConfigBooleanVyG3AEThAccessLogAccessLogAccessLogMachineRuntimeConfigBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerBase64StringUnsignedIntegerUnsignedIntegerUnsignedIntegerBase64StringMemoryRangeConfigCSRCSRUnsignedIntegerCSRUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerStringDoaGddGABooleanVyG3AEThSemanticVersionBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThInterpreterBreakReasonUarchInterpreterBreakReasonAccessLogBooleanVyG3AEThBooleanVyG3AEThProofBase64HashProofUnsignedIntegerBase64StringBooleanVyG3AEThBase64StringBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThMachineConfigMachineConfigBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AETh
{
    MachineConfig(MachineConfig),
    MachineRuntimeConfig(MachineRuntimeConfig),
    StringDoaGddGA(StringDoaGddGA),
    UnsignedInteger(UnsignedInteger),
    AccessLogType(AccessLogType),
    BooleanVyG3AETh(BooleanVyG3AETh),
    AccessLog(AccessLog),
    Base64String(Base64String),
    MemoryRangeConfig(MemoryRangeConfig),
    CSR(CSR),
    SemanticVersion(SemanticVersion),
    InterpreterBreakReason(InterpreterBreakReason),
    UarchInterpreterBreakReason(UarchInterpreterBreakReason),
    Proof(Proof),
    Base64Hash(Base64Hash),
}
#[derive(Clone)]
pub struct RemoteCartesiMachine<T> {
    transport: Box<T>,
}

#[allow(non_snake_case)]
impl<T: jsonrpsee::core::client::ClientT + Send + Sync> RemoteCartesiMachine<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(transport: T) -> Self {
        RemoteCartesiMachine {
            transport: Box::new(transport),
        }
    }

    pub async fn CheckConnection<'a>(&'a self) -> Result<(), Error> {
        let method = "";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn Fork<'a>(&'a self) -> Result<String, Error> {
        let method = "fork";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn Shutdown<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "shutdown";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn GetVersion<'a>(&'a self) -> Result<SemanticVersion, Error> {
        let method = "get_version";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineMachineConfig<'a>(
        &'a self,
        config: MachineConfig,
        runtime: MachineRuntimeConfig,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.machine.config";
        let mut params = ArrayParams::new();
        params.insert(config)?;
        params.insert(runtime)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineMachineDirectory<'a>(
        &'a self,
        directory: StringDoaGddGA,
        runtime: MachineRuntimeConfig,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.machine.directory";
        let mut params = ArrayParams::new();
        params.insert(directory)?;
        params.insert(runtime)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineDestroy<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.destroy";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineStore<'a>(
        &'a self,
        directory: StringDoaGddGA,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.store";
        let mut params = ArrayParams::new();
        params.insert(directory)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineRun<'a>(
        &'a self,
        mcycle_end: UnsignedInteger,
    ) -> Result<InterpreterBreakReason, Error> {
        let method = "machine.run";
        let mut params = ArrayParams::new();
        params.insert(mcycle_end)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineRunUarch<'a>(
        &'a self,
        uarch_cycle_end: UnsignedInteger,
    ) -> Result<UarchInterpreterBreakReason, Error> {
        let method = "machine.run_uarch";
        let mut params = ArrayParams::new();
        params.insert(uarch_cycle_end)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineStepUarch<'a>(
        &'a self,
        log_type: AccessLogType,
        one_based: BooleanVyG3AETh,
    ) -> Result<AccessLog, Error> {
        let method = "machine.log_uarch_step";
        let mut params = ArrayParams::new();
        params.insert(log_type)?;
        params.insert(one_based)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineVerifyUarchAccessLog<'a>(
        &'a self,
        log: AccessLog,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_uarch_step_log";
        let mut params = ArrayParams::new();
        params.insert(log)?;
        params.insert(runtime)?;
        params.insert(one_based)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineVerifyUarchStepStateTransition<'a>(
        &'a self,
        root_hash_before: String,
        log: AccessLog,
        root_hash_after: String,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_uarch_step_state_transition";
        let mut params = ArrayParams::new();
        params.insert(root_hash_before)?;
        params.insert(log)?;
        params.insert(root_hash_after)?;
        params.insert(runtime)?;
        params.insert(one_based)?;
        self.transport.request(method, params).await
    }


    pub async fn MachineVerifyAccessLog<'a>(
        &'a self,
        log: AccessLog,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_step_log";
        let mut params = ArrayParams::new();
        params.insert(log)?;
        params.insert(runtime)?;
        params.insert(one_based)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineVerifyStepStateTransition<'a>(
        &'a self,
        root_hash_before: String,
        log: AccessLog,
        root_hash_after: String,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_step_state_transition";
        let mut params = ArrayParams::new();
        params.insert(root_hash_before)?;
        params.insert(log)?;
        params.insert(root_hash_after)?;
        params.insert(runtime)?;
        params.insert(one_based)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetProof<'a>(
        &'a self,
        address: UnsignedInteger,
        log2_size: UnsignedInteger,
    ) -> Result<Proof, Error> {
        let method = "machine.get_proof";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        params.insert(log2_size)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetRootHash<'a>(&'a self) -> Result<Base64Hash, Error> {
        let method = "machine.get_root_hash";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadWord<'a>(
        &'a self,
        address: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_word";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadMemory<'a>(
        &'a self,
        address: UnsignedInteger,
        length: UnsignedInteger,
    ) -> Result<Base64String, Error> {
        let method = "machine.read_memory";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        params.insert(length)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteMemory<'a>(
        &'a self,
        address: UnsignedInteger,
        data: Base64String,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_memory";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        params.insert(data)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadVirtualMemory<'a>(
        &'a self,
        address: UnsignedInteger,
        length: UnsignedInteger,
    ) -> Result<Base64String, Error> {
        let method = "machine.read_virtual_memory";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        params.insert(length)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteVirtualMemory<'a>(
        &'a self,
        address: UnsignedInteger,
        data: Base64String,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_virtual_memory";
        let mut params = ArrayParams::new();
        params.insert(address)?;
        params.insert(data)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReplaceMemoryRange<'a>(
        &'a self,
        range: MemoryRangeConfig,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.replace_memory_range";
        let mut params = ArrayParams::new();
        params.insert(range)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadCsr<'a>(&'a self, csr: String) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_csr";
        let mut params = ArrayParams::new();
        params.insert(csr)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteCsr<'a>(
        &'a self,
        csr: String,
        value: UnsignedInteger,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_csr";
        let mut params = ArrayParams::new();
        params.insert(csr)?;
        params.insert(value)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetCsrAddress<'a>(&'a self, csr: String) -> Result<UnsignedInteger, Error> {
        let method = "machine.get_csr_address";
        let mut params = ArrayParams::new();
        params.insert(csr)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadX<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_x";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadF<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_f";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineReadUarchX<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_uarch_x";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteX<'a>(
        &'a self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_x";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        params.insert(value)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteF<'a>(
        &'a self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_f";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        params.insert(value)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineWriteUarchX<'a>(
        &'a self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.write_uarch_x";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        params.insert(value)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetXAddress<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.get_x_address";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetFAddress<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.get_f_address";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineGetUarchXAddress<'a>(
        &'a self,
        index: UnsignedInteger,
    ) -> Result<UnsignedInteger, Error> {
        let method = "machine.get_uarch_x_address";
        let mut params = ArrayParams::new();
        params.insert(index)?;
        self.transport.request(method, params).await
    }

    pub async fn MachineSetIflagsY<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.set_iflags_Y";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineResetIflagsY<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.reset_iflags_Y";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadIflagsY<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.read_iflags_Y";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineSetIflagsX<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.set_iflags_X";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineResetIflagsX<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.reset_iflags_X";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadIflagsX<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.read_iflags_X";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineSetIflagsH<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.set_iflags_H";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadIflagsH<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.read_iflags_H";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadIflagsPRV<'a>(&'a self) -> Result<UnsignedInteger, Error> {
        let method = "machine.read_iflags_PRV";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineSetUarchHaltFlag<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.set_uarch_halt_flag";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineReadUarchHaltFlag<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.read_uarch_halt_flag";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineResetUarchState<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.reset_uarch_state";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineGetInitialConfig<'a>(&'a self) -> Result<MachineConfig, Error> {
        let method = "machine.get_initial_config";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineGetDefaultConfig<'a>(&'a self) -> Result<MachineConfig, Error> {
        let method = "machine.get_default_config";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineVerifyMerkleTree<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_merkle_tree";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }

    pub async fn MachineVerifyDirtyPageMaps<'a>(&'a self) -> Result<BooleanVyG3AETh, Error> {
        let method = "machine.verify_dirty_page_maps";
        let params = ArrayParams::new();
        self.transport.request(method, params).await
    }
}
