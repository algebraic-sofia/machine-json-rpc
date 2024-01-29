// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

//! Implementation of the Rust json-rpc client for Cartesi emulator machine server

#![allow(unused_variables, dead_code)]

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
pub use jsonrpsee::core::Error;

use crate::interfaces::{self, Base64Hash};

mod conversions;
use conversions::*;

#[doc = " Server version"]
#[derive(Debug, Clone, Default)]
pub struct SemanticVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_release: String,
    pub build: String,
}

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        (
            self.major,
            self.minor,
            self.patch,
            &self.pre_release,
            &self.build,
        ) == (
            other.major,
            other.minor,
            other.patch,
            &other.pre_release,
            &other.build,
        )
    }
}

impl Eq for SemanticVersion {}

impl From<&interfaces::SemanticVersion> for SemanticVersion {
    fn from(version: &interfaces::SemanticVersion) -> Self {
        SemanticVersion {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            pre_release: version.pre_release.clone().unwrap_or_default(),
            build: version.build.clone().unwrap_or_default(),
        }
    }
}

#[doc = " Cartesi machine processor state configuration"]
#[derive(Debug, Copy, Clone, Default)]
pub struct ProcessorConfig {
    #[doc = "< Value of general-purpose registers"]
    pub x: [u64; 32usize],
    #[doc = "< Value of f registers"]
    pub f: [u64; 32usize],
    #[doc = "< Value of pc"]
    pub pc: u64,
    #[doc = "< Value of mvendorid CSR"]
    pub mvendorid: u64,
    #[doc = "< Value of marchid CSR"]
    pub marchid: u64,
    #[doc = "< Value of mimpid CSR"]
    pub mimpid: u64,
    #[doc = "< Value of mcycle CSR"]
    pub mcycle: u64,
    #[doc = "< Value of icycleinstret CSR"]
    pub icycleinstret: u64,
    #[doc = "< Value of mstatus CSR"]
    pub mstatus: u64,
    #[doc = "< Value of mtvec CSR"]
    pub mtvec: u64,
    #[doc = "< Value of mscratch CSR"]
    pub mscratch: u64,
    #[doc = "< Value of mepc CSR"]
    pub mepc: u64,
    #[doc = "< Value of mcause CSR"]
    pub mcause: u64,
    #[doc = "< Value of mtval CSR"]
    pub mtval: u64,
    #[doc = "< Value of misa CSR"]
    pub misa: u64,
    #[doc = "< Value of mie CSR"]
    pub mie: u64,
    #[doc = "< Value of mip CSR"]
    pub mip: u64,
    #[doc = "< Value of medeleg CSR"]
    pub medeleg: u64,
    #[doc = "< Value of mideleg CSR"]
    pub mideleg: u64,
    #[doc = "< Value of mcounteren CSR"]
    pub mcounteren: u64,
    #[doc = "< Value of stvec CSR"]
    pub stvec: u64,
    #[doc = "< Value of sscratch CSR"]
    pub sscratch: u64,
    #[doc = "< Value of sepc CSR"]
    pub sepc: u64,
    #[doc = "< Value of scause CSR"]
    pub scause: u64,
    #[doc = "< Value of stval CSR"]
    pub stval: u64,
    #[doc = "< Value of satp CSR"]
    pub satp: u64,
    #[doc = "< Value of scounteren CSR"]
    pub scounteren: u64,
    #[doc = "< Value of ilrsc CSR"]
    pub ilrsc: u64,
    #[doc = "< Value of iflags CSR"]
    pub iflags: u64,
    #[doc = "< Value of senvcfg CSR"]
    pub senvcfg: u64,
    #[doc = "< Value of menvcfg CSR"]
    pub menvcfg: u64,
    #[doc = "< Value of fcsr CSR"]
    pub fcsr: u64,
}

impl ProcessorConfig {
    pub fn new() -> Self {
        ProcessorConfig {
            mvendorid: 0x6361727465736920,
            marchid: 0x7,
            mimpid: 0x1,
            ..Default::default()
        }
    }
}

impl From<&interfaces::ProcessorConfig> for ProcessorConfig {
    fn from(config: &interfaces::ProcessorConfig) -> Self {
        ProcessorConfig {
            x: convert_x_csr_field(&config.x),
            f: convert_f_csr_field(&config.f),
            pc: convert_csr_field(config.pc),
            mvendorid: convert_csr_field(config.mvendorid),
            marchid: convert_csr_field(config.marchid),
            mimpid: convert_csr_field(config.mimpid),
            mcycle: convert_csr_field(config.mcycle),
            icycleinstret: convert_csr_field(config.icycleinstret),
            mstatus: convert_csr_field(config.mstatus),
            mtvec: convert_csr_field(config.mtvec),
            mscratch: convert_csr_field(config.mscratch),
            mepc: convert_csr_field(config.mepc),
            mcause: convert_csr_field(config.mcause),
            mtval: convert_csr_field(config.mtval),
            misa: convert_csr_field(config.misa),
            mie: convert_csr_field(config.mie),
            mip: convert_csr_field(config.mip),
            medeleg: convert_csr_field(config.medeleg),
            mideleg: convert_csr_field(config.mideleg),
            mcounteren: convert_csr_field(config.mcounteren),
            stvec: convert_csr_field(config.stvec),
            sscratch: convert_csr_field(config.sscratch),
            sepc: convert_csr_field(config.sepc),
            scause: convert_csr_field(config.scause),
            stval: convert_csr_field(config.stval),
            satp: convert_csr_field(config.satp),
            scounteren: convert_csr_field(config.scounteren),
            ilrsc: convert_csr_field(config.ilrsc),
            iflags: convert_csr_field(config.iflags),
            senvcfg: convert_csr_field(config.senvcfg),
            menvcfg: convert_csr_field(config.menvcfg),
            fcsr: convert_csr_field(config.fcsr),
        }
    }
}

#[doc = " Cartesi machine RAM state configuration"]
#[derive(Debug, Clone, Default)]
pub struct RamConfig {
    #[doc = "< RAM length"]
    pub length: u64,
    #[doc = "< RAM image file name"]
    pub image_filename: String,
}

impl RamConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::RAMConfig> for RamConfig {
    fn from(config: &interfaces::RAMConfig) -> Self {
        RamConfig {
            length: config.length,
            image_filename: config.image_filename.clone().unwrap_or_default(),
        }
    }
}

#[doc = " Cartesi machine Tlb"]
#[derive(Debug, Clone, Default)]
pub struct TlbConfig {
    #[doc = "< Tlb image file name"]
    pub image_filename: String,
}

impl TlbConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::TLBConfig> for TlbConfig {
    fn from(config: &interfaces::TLBConfig) -> Self {
        TlbConfig {
            image_filename: config.image_filename.clone().unwrap_or_default(),
        }
    }
}

#[doc = " Cartesi machine Uarch"]
#[derive(Debug, Clone, Default)]
pub struct UarchConfig {
    #[doc = "< Uarch processor"]
    pub processor: ::core::option::Option<interfaces::UarchProcessorConfig>,
    #[doc = "< Uarch ram"]
    pub ram: ::core::option::Option<interfaces::UarchRAMConfig>,
}

impl UarchConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::UarchConfig> for UarchConfig {
    fn from(config: &interfaces::UarchConfig) -> Self {
        UarchConfig {
            processor: config.processor.clone(),
            ram: config.ram.clone(),
        }
    }
}

#[doc = " Cartesi machine ROM state configuration"]
#[derive(Debug, Clone, Default)]
pub struct RomConfig {
    #[doc = "< Bootargs to pass to kernel"]
    pub bootargs: String,
    #[doc = "< ROM image file"]
    pub image_filename: String,
}

impl RomConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::ROMConfig> for RomConfig {
    fn from(config: &interfaces::ROMConfig) -> Self {
        RomConfig {
            image_filename: config.image_filename.clone().unwrap_or_default(),
            bootargs: config.bootargs.clone().unwrap_or_default(),
        }
    }
}

#[doc = " Cartesi machine memory range state configuration"]
#[derive(Debug, Clone, Default)]
pub struct MemoryRangeConfig {
    #[doc = "< Memory range start position"]
    pub start: u64,
    #[doc = "< Memory range length"]
    pub length: u64,
    #[doc = "< Target changes to drive affect image file?"]
    pub shared: bool,
    #[doc = "< Memory range image file name"]
    pub image_filename: String,
}

impl MemoryRangeConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::MemoryRangeConfig> for MemoryRangeConfig {
    fn from(config: &interfaces::MemoryRangeConfig) -> Self {
        MemoryRangeConfig {
            start: config.start.unwrap_or_default(),
            length: config.length.unwrap_or_default(),
            shared: config.shared.unwrap_or_default(),
            image_filename: config.image_filename.clone().unwrap_or_default(),
        }
    }
}

#[doc = " Cartesi machine rollup configuration"]
#[derive(Debug, Clone, Default)]
pub struct RollupConfig {
    pub rx_buffer: Option<MemoryRangeConfig>,
    pub tx_buffer: Option<MemoryRangeConfig>
}

impl RollupConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&interfaces::RollupConfig> for RollupConfig {
    fn from(config: &interfaces::RollupConfig) -> Self {
        RollupConfig {
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
        }
    }
}

#[doc = " Machine state configuration"]
#[derive(Debug, Clone, Default)]
pub struct MachineConfig {
    pub processor: ProcessorConfig,
    pub ram: RamConfig,
    pub rom: RomConfig,
    pub flash_drives: Vec<MemoryRangeConfig>,
    pub clint: interfaces::CLINTConfig,
    pub htif: interfaces::HTIFConfig,
    pub rollup: RollupConfig,
    pub tlb: TlbConfig,
    pub uarch: UarchConfig,
}

impl From<&interfaces::MachineConfig> for MachineConfig {
    fn from(mc: &interfaces::MachineConfig) -> Self {
        MachineConfig {
            processor: match &mc.processor {
                Some(proc_config) => ProcessorConfig::from(proc_config),
                None => ProcessorConfig::new(),
            },
            ram: match &mc.ram {
                Some(ram_config) => RamConfig::from(ram_config),
                None => RamConfig::new(),
            },
            rom: match &mc.rom {
                Some(rom_config) => RomConfig::from(rom_config),
                None => RomConfig::new(),
            },
            tlb: match &mc.tlb {
                Some(tlb_config) => TlbConfig::from(tlb_config),
                None => TlbConfig::new(),
            },
            uarch: match &mc.uarch {
                Some(uarch_config) => UarchConfig::from(uarch_config),
                None => UarchConfig::new(),
            },
            flash_drives: {
                mc.flash_drive
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(MemoryRangeConfig::from)
                    .collect()
            },
            clint: match &mc.clint {
                Some(clint_config) => interfaces::CLINTConfig::from(clint_config.clone()),
                None => Default::default(),
            },
            htif: match &mc.htif {
                Some(htif_config) => interfaces::HTIFConfig::from(htif_config.clone()),
                None => Default::default(),
            },
            rollup: match &mc.rollup {
                Some(rollup_config) => RollupConfig::from(rollup_config),
                None => RollupConfig::new(),
            },
        }
    }
}

#[doc = " Concurrency runtime configuration"]
#[derive(Debug, Clone, Default)]
pub struct ConcurrencyConfig {
    pub update_merkle_tree: u64,
}

impl From<&interfaces::ConcurrencyConfig> for ConcurrencyConfig {
    fn from(config: &interfaces::ConcurrencyConfig) -> Self {
        ConcurrencyConfig {
            update_merkle_tree: config.update_merkle_tree.unwrap_or_default(),
        }
    }
}

#[doc = " Machine runtime configuration"]
#[derive(Debug, Clone, Default)]
pub struct MachineRuntimeConfig {
    pub concurrency: ConcurrencyConfig,
    pub htif: interfaces::HTIFRuntimeConfig,
    pub skip_root_hash_check: bool,
    pub skip_version_check: bool,
}

impl From<&interfaces::MachineRuntimeConfig> for MachineRuntimeConfig {
    fn from(rc: &interfaces::MachineRuntimeConfig) -> Self {
        MachineRuntimeConfig {
            concurrency: ConcurrencyConfig::from(
                rc.concurrency
                    .as_ref()
                    .unwrap_or(&interfaces::ConcurrencyConfig::default()),
            ),
            htif: rc
                .htif
                .clone()
                .unwrap_or(interfaces::HTIFRuntimeConfig::default()),
            skip_root_hash_check: rc.skip_root_hash_check.unwrap_or(false),
            skip_version_check: rc.skip_version_check.unwrap_or(false),
        }
    }
}

#[doc = " Merkle tree proof structure"]
#[doc = " \\details"]
#[doc = " This structure holds a proof that the node spanning a log2_target_size"]
#[doc = " at a given address in the tree has a certain hash."]
#[derive(Debug, Clone, Default)]
pub struct MerkleTreeProof {
    pub target_address: u64,
    pub log2_target_size: usize,
    pub target_hash: Base64Hash,
    pub log2_root_size: usize,
    pub root_hash: Base64Hash,
    pub sibling_hashes: Vec<Base64Hash>,
}

impl From<&interfaces::Proof> for MerkleTreeProof {
    fn from(proof: &interfaces::Proof) -> Self {
        MerkleTreeProof {
            target_address: proof.target_address,
            log2_target_size: proof.log_2_target_size as usize,
            log2_root_size: proof.log_2_root_size as usize,
            target_hash: proof.target_hash.clone(),
            root_hash: proof.root_hash.clone(),
            sibling_hashes: proof.sibling_hashes.clone(),
        }
    }
}

#[doc = " Type of state access"]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessType {
    Read = 0,
    Write,
}

#[doc = " Access log type"]
#[derive(Debug, Clone, Copy, Default)]
pub struct AccessLogType {
    pub proofs: bool,
    pub annotations: bool,
    pub has_large_data: bool,
}

impl From<&interfaces::AccessLogType> for AccessLogType {
    fn from(log_type: &interfaces::AccessLogType) -> Self {
        AccessLogType {
            proofs: log_type.has_proofs,
            annotations: log_type.has_annotations,
            has_large_data: log_type.has_large_data,
        }
    }
}

#[doc = " Records an access to the machine state"]
#[derive(Debug, Clone)]
pub struct Access {
    #[doc = "< Type of access"]
    pub r#type: AccessType,
    #[doc = "< Address of access"]
    pub address: u64,
    #[doc = "< Log2 of size of access"]
    pub log2_size: i32,
    #[doc = "< Data before access"]
    pub read_data: Vec<u8>,
    #[doc = "< Data after access (if writing)"]
    pub written_data: Vec<u8>,
    #[doc = "< Proof of data before access"]
    pub proof: MerkleTreeProof,
}

impl From<&interfaces::Access> for Access {
    fn from(access: &interfaces::Access) -> Self {
        let mut read_data = access.read.clone();
        let mut written_data: String = match access.written.clone() {
            Some(written_data) => written_data.clone(),
            None => Default::default(),
        };

        if written_data.ends_with('\n') {
            written_data.pop();
        }

        if read_data.ends_with('\n') {
            read_data.pop();
        }

        Access {
            r#type: match access.r#type.to_string().as_str() {
                "\"read\"" => AccessType::Read,
                "\"write\"" => AccessType::Write,
                _ => AccessType::Read,
            },
            read_data: STANDARD.decode(read_data).unwrap(),
            written_data: STANDARD.decode(written_data).unwrap(),
            proof: match &access.proof {
                Some(x) => MerkleTreeProof::from(x),
                None => Default::default(),
            },
            address: access.address,
            log2_size: access.log_2_size as i32,
        }
    }
}

#[doc = " Bracket type"]
#[derive(Debug, Clone, Copy)]
pub enum BracketType {
    Begin = 0,
    End,
}

#[doc = " Bracket note"]
#[derive(Debug, Clone)]
pub struct BracketNote {
    #[doc = "< Bracket type"]
    pub r#type: BracketType,
    #[doc = "< Where it points to in the log"]
    pub r#where: u64,
    #[doc = "< Note text"]
    pub text: String,
}
impl std::convert::From<&interfaces::Bracket> for BracketNote {
    fn from(bracket_note: &interfaces::Bracket) -> Self {
        BracketNote {
            r#type: match bracket_note.r#type.to_string().as_str() {
                "begin" => BracketType::Begin,
                "end" => BracketType::End,
                _ => BracketType::Begin,
            },
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

#[doc = " Access log"]
#[derive(Debug, Clone)]
pub struct AccessLog {
    pub accesses: Vec<Access>,
    pub brackets: Vec<BracketNote>,
    pub notes: Vec<String>,
    pub log_type: AccessLogType,
    pub has_large_data: bool,
}

impl From<&interfaces::AccessLog> for AccessLog {
    fn from(log: &interfaces::AccessLog) -> Self {
        let log_type = AccessLogType {
            proofs: log.log_type.has_proofs,
            annotations: log.log_type.has_annotations,
            has_large_data: log.log_type.has_large_data,
        };
        AccessLog {
            log_type,
            accesses: log.accesses.iter().map(Access::from).collect(),
            brackets: log
                .brackets
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|e| BracketNote::from(e))
                .collect(),
            notes: log.notes.clone().unwrap_or_default(),
            has_large_data: log.has_large_data,
        }
    }
}

#[doc = "Client for Cartesi emulator machine server"]
#[derive(Clone)]

pub struct JsonRpcCartesiMachineClient {
    server_address: String,
    client: interfaces::RemoteCartesiMachine<jsonrpsee::http_client::HttpClient>,
}

impl JsonRpcCartesiMachineClient {
    /// Create new client instance. Connect to the server as part of client instantiation
    pub async fn new<'a>(server_address: String) -> Result<Self, Error> {
        let transport = jsonrpsee::http_client::HttpClientBuilder::default()
            .request_timeout(core::time::Duration::MAX)
            .build(&server_address)?;

        let remote_machine = interfaces::RemoteCartesiMachine::new(transport);
        remote_machine.GetVersion().await?;
        // somehow CheckConnection won't work
        // remote_machine.CheckConnection().await

        Ok(JsonRpcCartesiMachineClient {
            server_address,
            client: remote_machine,
        })
    }

    /// Create new client instance. Connect to the server as part of client instantiation
    pub fn get_address(&self) -> &String {
        &self.server_address
    }

    /// Get Cartesi machine server version
    pub async fn get_version(&self) -> Result<SemanticVersion, Error> {
        self.client
            .GetVersion()
            .await
            .map(|res| SemanticVersion::from(&res))
    }

    // /// Create machine instance on remote Cartesi machine server
    pub async fn create_machine(
        &self,
        machine_config: &MachineConfig,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<bool, Error> {
        let runtime = interfaces::MachineRuntimeConfig::from(machine_runtime_config);
        let machine_oneof = interfaces::MachineConfig::from(machine_config);
        self.client
            .MachineMachineConfig(machine_oneof, runtime)
            .await
    }

    /// Create machine from storage on remote Cartesi machine server
    pub async fn load_machine(
        &self,
        directory: &str,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<bool, Error> {
        let runtime = interfaces::MachineRuntimeConfig::from(machine_runtime_config);
        self.client
            .MachineMachineDirectory(directory.to_string(), runtime)
            .await
    }

    /// Run remote machine to maximum limit cycle
    pub async fn run(&self, limit: u64) -> Result<serde_json::Value, Error> {
        self.client.MachineRun(limit).await
    }

    /// Run uarch remote machine to maximum limit cycle
    pub async fn run_uarch(&self, limit: u64) -> Result<serde_json::Value, Error> {
        self.client.MachineRunUarch(limit).await
    }

    /// Serialize entire remote machine state to directory on cartesi machine server host
    pub async fn store(&self, directory: &str) -> Result<bool, Error> {
        self.client.MachineStore(directory.to_string()).await
    }

    /// Destroy remote machine instance
    pub async fn destroy(&self) -> Result<bool, Error> {
        self.client.MachineDestroy().await
    }

    /// Fork remote machine
    pub async fn fork(&self) -> Result<String, Error> {
        self.client.Fork().await
    }

    /// Shutdown the server
    pub async fn shutdown(&self) -> Result<bool, Error> {
        self.client.Shutdown().await
    }

    /// Runs the remote machine for one cycle logging all accesses to the state
    pub async fn step(
        &self,
        log_type: &AccessLogType,
        one_based: bool,
    ) -> Result<AccessLog, Error> {
        let log_type = interfaces::AccessLogType {
            has_proofs: log_type.proofs,
            has_annotations: log_type.annotations,
            has_large_data: log_type.has_large_data
        };
        self.client
            .MachineStepUarch(log_type, one_based)
            .await
            .map(|op| AccessLog::from(&op))
    }

    /// Reads a chunk of data from the remote machine memory
    pub async fn read_memory(&self, address: u64, length: u64) -> Result<Vec<u8>, Error> {
        let response = self.client.MachineReadMemory(address, length).await?;

        let response = response.replace("\n", "");

        Ok(STANDARD.decode(response).unwrap())
    }

    /// Writes a chunk of data to the remote machine memory
    pub async fn write_memory(&self, address: u64, data: String) -> Result<bool, Error> {
        self.client.MachineWriteMemory(address, data).await
    }

    /// Read the value of a word in the remote machine state
    pub async fn read_word(&self, address: u64) -> Result<u64, Error> {
        self.client.MachineReadWord(address).await
    }

    /// Obtains the root hash of the Merkle tree for the remote machine
    pub async fn get_root_hash(&self) -> Result<[u8; 32], Error> {
        self.client.MachineGetRootHash().await.map(|op| {
            let mut hash = op;
            if hash.ends_with('\n') {
                hash.pop();
            }
            let mut root_hash = [0u8; 32];
            STANDARD
                .decode_slice_unchecked(hash.clone(), &mut root_hash as &mut [u8])
                .unwrap();
            root_hash
        })
    }

    /// Obtains the proof for a node in the Merkle tree from remote machine
    pub async fn get_proof(&self, address: u64, log2_size: u64) -> Result<MerkleTreeProof, Error> {
        self.client
            .MachineGetProof(address, log2_size)
            .await
            .map(|op| MerkleTreeProof::from(&op))
    }

    /// Replaces a flash drive on a remote machine
    pub async fn replace_memory_range(
        &self,
        config: interfaces::MemoryRangeConfig,
    ) -> Result<bool, Error> {
        self.client.MachineReplaceMemoryRange(config).await
    }

    /// Gets the address of a general-purpose register
    pub async fn get_x_address(&self, index: u64) -> Result<u64, Error> {
        self.client.MachineGetXAddress(index).await
    }

    /// Reads the value of a general-purpose register from the remote machine
    pub async fn read_x(&self, index: u64) -> Result<u64, Error> {
        self.client.MachineReadX(index).await
    }

    pub async fn read_iflags_h(&self) -> Result<bool, Error> {
        self.client.MachineReadIflagsH().await
    }

    pub async fn read_iflags_x(&self) -> Result<bool, Error> {
        self.client.MachineReadIflagsX().await
    }

    pub async fn read_iflags_y(&self) -> Result<bool, Error> {
        self.client.MachineReadIflagsY().await
    }

    pub async fn read_uarch_halt_flag(&self) -> Result<bool, Error> {
        self.client.MachineReadUarchHaltFlag().await
    }

    /// Writes the value of a general-purpose register for the remote machine
    pub async fn write_x(&self, index: u64, value: u64) -> Result<bool, Error> {
        self.client.MachineWriteX(index, value).await
    }

    /// Resets the value of the iflags_Y flag on the remote machine
    pub async fn reset_iflags_y(&self) -> Result<bool, Error> {
        self.client.MachineResetIflagsY().await
    }

    /// Resets uarch state on the remote machine
    pub async fn reset_uarch_state(&self) -> Result<bool, Error> {
        self.client.MachineResetUarchState().await
    }

    /// Gets the address of any CSR
    pub async fn get_csr_address(&self, csr: String) -> Result<u64, Error> {
        self.client.MachineGetCsrAddress(csr).await
    }

    /// Read the value of any CSR from remote machine
    pub async fn read_csr(&self, csr: String) -> Result<u64, Error> {
        self.client.MachineReadCsr(csr).await
    }

    /// Writes the value of any CSR on remote machine
    pub async fn write_csr(&self, csr: String, value: u64) -> Result<bool, Error> {
        self.client.MachineWriteCsr(csr, value).await
    }

    /// Returns copy of initialization config of the remote machine
    pub async fn get_initial_config(&self) -> Result<MachineConfig, Error> {
        self.client
            .MachineGetInitialConfig()
            .await
            .map(|op| MachineConfig::from(&op))
    }

    /// Verifies integrity of Merkle tree on the remote machine
    pub async fn verify_merkle_tree(&self) -> Result<bool, Error> {
        self.client.MachineVerifyMerkleTree().await
    }

    /// Verify if dirty page maps are consistent on the remote machine
    pub async fn verify_dirty_page_maps(&self) -> Result<bool, Error> {
        self.client.MachineVerifyDirtyPageMaps().await
    }

    /// Returns copy of default system config from remote Cartesi machine server
    pub async fn get_default_config(&self) -> Result<MachineConfig, Error> {
        self.client
            .MachineGetDefaultConfig()
            .await
            .map(|op| MachineConfig::from(&op))
    }

    /// Checks the internal consistency of an access log
    pub async fn verify_uarch_access_log(
        &self,
        log: &AccessLog,
        runtime: &MachineRuntimeConfig,
        one_based: bool,
    ) -> Result<bool, Error> {
        let log = interfaces::AccessLog::from(log);
        let runtime = interfaces::MachineRuntimeConfig::from(runtime);

        self.client
            .MachineVerifyUarchAccessLog(log, runtime, one_based)
            .await
    }

    /// Checks the interal consistency of the log
    pub async fn verify_access_log(
        &self,
        log: &AccessLog,
        runtime: &MachineRuntimeConfig,
        one_based: bool,
    ) -> Result<bool, Error> {
        let log = interfaces::AccessLog::from(log);
        let runtime = interfaces::MachineRuntimeConfig::from(runtime);

        self.client
            .MachineVerifyAccessLog(log, runtime, one_based)
            .await
    }

    /// Checks the validity of a state transition
    pub async fn verify_state_transition(
        &self,
        root_hash_before: Vec<u8>,
        log: &AccessLog,
        root_hash_after: Vec<u8>,
        one_based: bool,
        runtime: &MachineRuntimeConfig,
    ) -> Result<bool, Error> {
        let mut root_hash_before = STANDARD.encode(&root_hash_before.clone());
        let mut root_hash_after = STANDARD.encode(&root_hash_after.clone());

        if root_hash_before.ends_with("=") {
            root_hash_before.push('\n');
        }
        if root_hash_after.ends_with("=") {
            root_hash_after.push('\n');
        }
        let log = interfaces::AccessLog::from(log);
        let runtime = interfaces::MachineRuntimeConfig::from(runtime);

        self.client
            .MachineVerifyStepStateTransition(
                root_hash_before,
                log,
                root_hash_after,
                runtime,
                one_based,
            )
            .await
    }

    /// Checks the validity of a state transition
    pub async fn verify_uarch_state_transition(
        &self,
        root_hash_before: Vec<u8>,
        log: &AccessLog,
        root_hash_after: Vec<u8>,
        one_based: bool,
        runtime: &MachineRuntimeConfig,
    ) -> Result<bool, Error> {
        let mut root_hash_before = STANDARD.encode(&root_hash_before.clone());
        let mut root_hash_after = STANDARD.encode(&root_hash_after.clone());

        if root_hash_before.ends_with("=") {
            root_hash_before.push('\n');
        }
        if root_hash_after.ends_with("=") {
            root_hash_after.push('\n');
        }
        let log = interfaces::AccessLog::from(log);
        let runtime = interfaces::MachineRuntimeConfig::from(runtime);

        self.client
            .MachineVerifyUarchStepStateTransition(
                root_hash_before,
                log,
                root_hash_after,
                runtime,
                one_based,
            )
            .await
    }
}
