#![allow(non_upper_case_globals)]

use crate::resource::ResourceType::*;
use magickwand_sys::*;

#[derive(Debug, PartialEq)]
pub enum ResourceType {
    Undefined,
    Area,
    Disk,
    File,
    Height,
    Map,
    Memory,
    Thread,
    Throttle,
    Time,
    Width,
    ListLength
}

impl From<magickwand_sys::ResourceType> for ResourceType {
    fn from(value: magickwand_sys::ResourceType) -> Self {
        match value {
            ResourceType_AreaResource => Area,
            ResourceType_DiskResource => Disk,
            ResourceType_FileResource => File,
            ResourceType_HeightResource => Height, 
            ResourceType_MapResource => Map,
            ResourceType_MemoryResource => Memory,
            ResourceType_ThreadResource => Thread,
            ResourceType_ThrottleResource => Throttle,
            ResourceType_TimeResource => Time,
            ResourceType_WidthResource => Width,
            ResourceType_ListLengthResource => ListLength,
            _ => Undefined
        }
    }
}

impl Into<magickwand_sys::ResourceType> for ResourceType {
    fn into(self) -> magickwand_sys::ResourceType {
        match self {
            Area => ResourceType_AreaResource,
            Disk => ResourceType_DiskResource,
            File => ResourceType_FileResource,
            Height => ResourceType_HeightResource, 
            Map => ResourceType_MapResource,
            Memory => ResourceType_MemoryResource,
            Thread => ResourceType_ThreadResource,
            Throttle => ResourceType_ThrottleResource,
            Time => ResourceType_TimeResource,
            Width => ResourceType_WidthResource,
            ListLength => ResourceType_ListLengthResource,
            _ => ResourceType_UndefinedResource
        }
    }
}
