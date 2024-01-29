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

use crate::client::*;
use crate::interfaces::{self, FRegConfig, XRegConfig};

impl From<&MachineRuntimeConfig> for interfaces::MachineRuntimeConfig {
    fn from(rc: &MachineRuntimeConfig) -> Self {
        interfaces::MachineRuntimeConfig {
            concurrency: Some(interfaces::ConcurrencyConfig {
                update_merkle_tree: Some(rc.concurrency.update_merkle_tree),
            }),
            htif: Some(interfaces::HTIFRuntimeConfig {
                no_console_putchar: rc.htif.no_console_putchar,
            }),
            skip_root_hash_check: Some(rc.skip_root_hash_check),
            skip_version_check: Some(rc.skip_version_check),
        }
    }
}

impl From<&MerkleTreeProof> for interfaces::Proof {
    fn from(proof: &MerkleTreeProof) -> Self {
        interfaces::Proof {
            target_address: proof.target_address,
            log_2_target_size: proof.log2_target_size as u64,
            log_2_root_size: proof.log2_root_size as u64,
            target_hash: proof.target_hash.clone(),
            root_hash: proof.root_hash.clone(),
            sibling_hashes: proof.sibling_hashes.clone(),
        }
    }
}

impl From<&Access> for interfaces::Access {
    fn from(access: &Access) -> Self {
        let mut read = STANDARD.encode(&access.read_data);
        let mut written = STANDARD.encode(&access.written_data);

        if read.ends_with("=") {
            read.push('\n');
        }
        if written.ends_with("=") {
            written.push('\n');
        }

        interfaces::Access {
            r#type: match access.r#type {
                AccessType::Read => serde_json::json!("read"),
                AccessType::Write => serde_json::json!("write"),
            },
            read: read,
            written: Some(written),
            proof: Some(interfaces::Proof::from(&access.proof)),
            address: access.address,
            log_2_size: access.log2_size as u64,
        }
    }
}

impl From<&BracketNote> for interfaces::Bracket {
    fn from(bracket_note: &BracketNote) -> Self {
        interfaces::Bracket {
            r#type: match bracket_note.r#type {
                BracketType::Begin => serde_json::json!("begin"),
                BracketType::End => serde_json::json!("end"),
            },
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

impl From<&AccessLogType> for interfaces::AccessLogType {
    fn from(log_type: &AccessLogType) -> Self {
        interfaces::AccessLogType {
            has_proofs: log_type.proofs,
            has_annotations: log_type.annotations,
            has_large_data: log_type.has_large_data,
        }
    }
}

impl From<&AccessLog> for interfaces::AccessLog {
    fn from(log: &AccessLog) -> Self {
        let log_type = interfaces::AccessLogType {
            has_proofs: log.log_type.proofs,
            has_annotations: log.log_type.annotations,
            has_large_data: log.log_type.has_large_data,
        };
        interfaces::AccessLog {
            log_type,
            accesses: log
                .accesses
                .iter()
                .map(|e| interfaces::Access::from(e))
                .collect(),
            brackets: Some(
                log.brackets
                    .iter()
                    .map(|e| interfaces::Bracket::from(e))
                    .collect(),
            ),
            notes: Some(log.notes.clone()),
            has_large_data: log.log_type.has_large_data,
        }
    }
}

pub fn convert_x_csr_field(x: &Option<XRegConfig>) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    if let Some(x_reg) = x {
        result
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = x_reg[i])
    }
    result
}

pub fn convert_f_csr_field(f: &Option<FRegConfig>) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    if let Some(f_reg) = f {
        result
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = f_reg[i])
    }
    result
}

pub fn convert_csr_field(field: ::core::option::Option<u64>) -> u64
where
{
    match field {
        Some(x) => u64::from(x),
        None => 0,
    }
}

impl From<&ProcessorConfig> for interfaces::ProcessorConfig {
    fn from(config: &ProcessorConfig) -> Self {
        interfaces::ProcessorConfig {
            x: Some(config.x.to_vec()),
            f: Some(config.f.to_vec()),
            pc: Some(config.pc),
            mvendorid: Some(config.mvendorid),
            marchid: Some(config.marchid),
            mimpid: Some(config.mimpid),
            mcycle: Some(config.mcycle),
            icycleinstret: Some(config.icycleinstret),
            mstatus: Some(config.mstatus),
            mtvec: Some(config.mtvec),
            mscratch: Some(config.mscratch),
            mepc: Some(config.mepc),
            mcause: Some(config.mcause),
            mtval: Some(config.mtval),
            misa: Some(config.misa),
            mie: Some(config.mie),
            mip: Some(config.mip),
            medeleg: Some(config.medeleg),
            mideleg: Some(config.mideleg),
            mcounteren: Some(config.mcounteren),
            stvec: Some(config.stvec),
            sscratch: Some(config.sscratch),
            sepc: Some(config.sepc),
            scause: Some(config.scause),
            stval: Some(config.stval),
            satp: Some(config.satp),
            scounteren: Some(config.scounteren),
            ilrsc: Some(config.ilrsc),
            iflags: Some(config.iflags),
            menvcfg: Some(config.menvcfg),
            senvcfg: Some(config.senvcfg),
            fcsr: Some(config.fcsr),
        }
    }
}

impl From<&RamConfig> for interfaces::RAMConfig {
    fn from(config: &RamConfig) -> Self {
        interfaces::RAMConfig {
            length: config.length,
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&RomConfig> for interfaces::ROMConfig {
    fn from(config: &RomConfig) -> Self {
        interfaces::ROMConfig {
            bootargs: Some(config.bootargs.clone()),
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&TlbConfig> for interfaces::TLBConfig {
    fn from(config: &TlbConfig) -> Self {
        interfaces::TLBConfig {
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&UarchConfig> for interfaces::UarchConfig {
    fn from(config: &UarchConfig) -> Self {
        interfaces::UarchConfig {
            processor: config.processor.clone(),
            ram: config.ram.clone(),
        }
    }
}

impl From<&MemoryRangeConfig> for interfaces::MemoryRangeConfig {
    fn from(config: &MemoryRangeConfig) -> Self {
        interfaces::MemoryRangeConfig {
            start: Some(config.start),
            length: Some(config.length),
            image_filename: Some(config.image_filename.clone()),
            shared: Some(config.shared),
        }
    }
}

impl From<&RollupConfig> for interfaces::RollupConfig {
    fn from(config: &RollupConfig) -> Self {
        interfaces::RollupConfig {
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
        }
    }
}

impl From<&MachineConfig> for interfaces::MachineConfig {
    fn from(config: &MachineConfig) -> Self {
        interfaces::MachineConfig {
            processor: Some(interfaces::ProcessorConfig::from(&config.processor)),
            ram: Some(interfaces::RAMConfig::from(&config.ram)),
            rom: Some(interfaces::ROMConfig::from(&config.rom)),
            tlb: Some(interfaces::TLBConfig::from(&config.tlb)),
            uarch: Some(interfaces::UarchConfig::from(&config.uarch)),
            flash_drive: Some(
                config
                    .flash_drives
                    .iter()
                    .map(|e| interfaces::MemoryRangeConfig::from(e))
                    .collect(),
            ),
            clint: Some(interfaces::CLINTConfig::from(config.clint.clone())),
            htif: Some(interfaces::HTIFConfig::from(config.htif.clone())),
            rollup: Some(interfaces::RollupConfig::from(&config.rollup)),
        }
    }
}
