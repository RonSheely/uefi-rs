// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::table::boot::{AllocateType, MemoryType};
use crate::{Handle, PhysicalAddress, Status};
use core::ffi::c_void;
use uguid::{Guid, guid};

newtype_enum! {
    /// Corresponds to the `EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH` enum.
    pub enum PciRootBridgeIoProtocolWidth: u32 => {
        UINT8 = 0,
        UINT16 = 1,
        UINT32 = 2,
        UINT64 = 3,
        FIFO_UINT8 = 4,
        FIFO_UINT16 = 5,
        FIFO_UINT32 = 6,
        FIFO_UINT64 = 7,
        FILL_UINT8 = 8,
        FILL_UINT16 = 9,
        FILL_UINT32 = 10,
        FILL_UINT64 = 11,
        MAXIMUM = 12,
    }
}

newtype_enum! {
    /// Corresponds to the `EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_OPERATION` enum.
    pub enum PciRootBridgeIoProtocolOperation: u32 => {
        BUS_MASTER_READ = 0,
        BUS_MASTER_WRITE = 1,
        BUS_MASTER_COMMON_BUFFER = 2,
        BUS_MASTER_READ64 = 3,
        BUS_MASTER_WRITE64 = 4,
        BUS_MASTER_COMMON_BUFFER64 = 5,
        MAXIMUM = 6,
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PciRootBridgeIoAccess {
    pub read: unsafe extern "efiapi" fn(
        this: *mut PciRootBridgeIoProtocol,
        width: PciRootBridgeIoProtocolWidth,
        address: u64,
        count: usize,
        buffer: *mut c_void,
    ) -> Status,
    pub write: unsafe extern "efiapi" fn(
        this: *mut PciRootBridgeIoProtocol,
        width: PciRootBridgeIoProtocolWidth,
        address: u64,
        count: usize,
        buffer: *const c_void,
    ) -> Status,
}

#[derive(Debug)]
#[repr(C)]
pub struct PciRootBridgeIoProtocol {
    pub parent_handle: Handle,
    pub poll_mem: unsafe extern "efiapi" fn(
        this: *mut Self,
        width: PciRootBridgeIoProtocolWidth,
        address: u64,
        mask: u64,
        value: u64,
        delay: u64,
        result: *mut u64,
    ) -> Status,
    pub poll_io: unsafe extern "efiapi" fn(
        this: *mut Self,
        width: PciRootBridgeIoProtocolWidth,
        address: u64,
        mask: u64,
        value: u64,
        delay: u64,
        result: *mut u64,
    ) -> Status,
    pub mem: PciRootBridgeIoAccess,
    pub io: PciRootBridgeIoAccess,
    pub pci: PciRootBridgeIoAccess,
    pub copy_mem: unsafe extern "efiapi" fn(
        this: *mut Self,
        width: PciRootBridgeIoProtocolWidth,
        dest_addr: u64,
        src_addr: u64,
        count: usize,
    ) -> Status,
    pub map: unsafe extern "efiapi" fn(
        this: *const Self,
        operation: PciRootBridgeIoProtocolOperation,
        host_addr: *const c_void,
        num_bytes: *mut usize,
        device_addr: *mut PhysicalAddress,
        mapping: *mut *mut c_void,
    ) -> Status,
    pub unmap: unsafe extern "efiapi" fn(this: *const Self, mapping: *const c_void) -> Status,
    pub allocate_buffer: unsafe extern "efiapi" fn(
        this: *const Self,
        alloc_ty: AllocateType,
        memory_ty: MemoryType,
        pages: usize,
        host_addr: *mut *const c_void,
        attributes: u64,
    ) -> Status,
    pub free_buffer: unsafe extern "efiapi" fn(
        this: *const Self,
        pages: usize,
        host_addr: *const c_void,
    ) -> Status,
    pub flush: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub get_attributes: unsafe extern "efiapi" fn(
        this: *const Self,
        supports: *mut u64,
        attributes: *mut u64,
    ) -> Status,
    pub set_attributes: unsafe extern "efiapi" fn(
        this: *mut Self,
        attributes: u64,
        resource_base: *mut u64,
        resource_length: *mut u64,
    ) -> Status,
    pub configuration:
        unsafe extern "efiapi" fn(this: *const Self, resources: *mut *const c_void) -> Status,
    pub segment_number: u32,
}

impl PciRootBridgeIoProtocol {
    pub const GUID: Guid = guid!("2f707ebb-4a1a-11d4-9a38-0090273fc14d");
}
