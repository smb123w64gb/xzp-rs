use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite)]
#[brw(little, magic = b"piZx")]
pub struct XZipHeaderT{
    pub version : u32,
    pub preload_directory_entries : u32,
    pub directory_entries : u32,
    pub preload_bytes : u32,
    pub header_length : u32,
    pub filename_entries : u32,
    pub filename_strings_offset : u32,
    pub filename_strings_length : u32,

}
#[derive(BinRead, BinWrite)]
pub struct XZipDirectoryEntryT{
    pub filename_crc:u32,
    pub length:u32,
    pub stored_offset:i32,
}
#[derive(BinRead, BinWrite)]
pub struct xZipFilenameEntryT{
    pub filename_crc:u32,
    pub filename_offset:u32,
    pub time_stamp:u32,
}
#[derive(BinRead, BinWrite)]
pub struct xZipFooterT{
    pub size:u32,
    pub magic:u32,
}
struct CXZip{
    m_Header : XZipHeaderT,
    m_pPreloadDirectory : Vec<XZipDirectoryEntryT>,
    m_pDirectory : Vec<XZipDirectoryEntryT>,
    m_pFilenames : Vec<xZipFilenameEntryT>,
    m_pPreloadedData : Vec<u8>,
    m_nPreloadStart : u32,
    m_nRegular2PreloadEntryMapping : Vec<u16>,

}
impl XZipDirectoryEntryT{
    pub fn sort_compare(l:XZipDirectoryEntryT,r:XZipDirectoryEntryT) -> i32{
        if l.filename_crc < r.filename_crc
	{
		return -1;
	}

	else if l.filename_crc > r.filename_crc
	{
		return 1;
	}

	// else l.FileOffset == r.FileOffset
	if l.length < r.length
	{
		return -1;
	}
	else if l.length > r.length
	{
		return 1;
	}

	// else l.length == r.length
	if l.stored_offset < r.stored_offset
	{
		return -1;
	}
	else if l.stored_offset > r.stored_offset
	{
		return 1;
	}

	// else everything is identical:
	0
    }
    pub fn find_compare(l:XZipDirectoryEntryT,r:XZipDirectoryEntryT) -> i32{
        if l.filename_crc < r.filename_crc
        {
            return -1;
        }
    
        else if l.filename_crc > r.filename_crc
        {
            return 1;
        }
    
        0
    }
}
impl CXZip{

}
