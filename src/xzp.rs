use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite)]
#[brw(little, magic = b"piZx")]
pub struct xZipHeader_t{
    pub Version : u32,
    pub PreloadDirectoryEntries : u32,
    pub DirectoryEntries : u32,
    pub PreloadBytes : u32,
    pub HeaderLength : u32,
    pub FilenameEntries : u32,
    pub FilenameStringsOffset : u32,
    pub FilenameStringsLength : u32,

}
#[derive(BinRead, BinWrite)]
pub struct xZipDirectoryEntry_t{
    pub FilenameCRC:u32,
    pub Length:u32,
    pub StoredOffset:i32,
}
impl xZipDirectoryEntry_t{
    pub fn SortCompare(l:xZipDirectoryEntry_t,r:xZipDirectoryEntry_t) -> i32{
        if l.FilenameCRC < r.FilenameCRC
	{
		return -1;
	}

	else if l.FilenameCRC > r.FilenameCRC
	{
		return 1;
	}

	// else l.FileOffset == r.FileOffset
	if l.Length < r.Length
	{
		return -1;
	}
	else if l.Length > r.Length
	{
		return 1;
	}

	// else l.Length == r.Length
	if l.StoredOffset < r.StoredOffset
	{
		return -1;
	}
	else if l.StoredOffset > r.StoredOffset
	{
		return 1;
	}

	// else everything is identical:
	0
    }
    pub fn FindCompare(l:xZipDirectoryEntry_t,r:xZipDirectoryEntry_t) -> i32{
        if l.FilenameCRC < r.FilenameCRC
        {
            return -1;
        }
    
        else if l.FilenameCRC > r.FilenameCRC
        {
            return 1;
        }
    
        0
    }
}
