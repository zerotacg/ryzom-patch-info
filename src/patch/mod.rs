use crate::pd;

#[derive(Debug)]
pub struct CProductDescriptionForClient {
    _Files: CBNPFileSet,          // read_struct
    _Categories: CBNPCategorySet, // read_struct
}

impl CProductDescriptionForClient {
    pub fn from(pdr: &mut pd::PersistentDataRecord) -> CProductDescriptionForClient {
        CProductDescriptionForClient {
            _Files: pdr.read("_Files"),
            _Categories: pdr.read("_Categories"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPFileSet {
    pub _Files: Vec<CBNPFile>, // read_struct_vec
}

impl pd::Readable for CBNPFileSet {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFileSet {
        CBNPFileSet {
            _Files: pdr.read("_Files"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPCategorySet {
    pub _Category: Vec<CBNPCategory>,
}

impl pd::Readable for CBNPCategorySet {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPCategorySet {
        CBNPCategorySet {
            _Category: pdr.read("_Category"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPCategory {
    pub _Name: String,
    pub _IsOptional: Option<bool>,
    pub _UnpackTo: Option<String>,
    pub _IsIncremental: Option<bool>,
    pub _CatRequired: Option<String>,
    pub _Hidden: Option<bool>,
    pub _Files: Vec<String>,
}

impl pd::Readable for CBNPCategory {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPCategory {
        CBNPCategory {
            _Name: pdr.read("_Name"),
            _IsOptional: pdr.read("_IsOptional"),
            _UnpackTo: pdr.read("_UnpackTo"),
            _IsIncremental: pdr.read("_IsIncremental"),
            _CatRequired: pdr.read("_CatRequired"),
            _Hidden: pdr.read("_Hidden"),
            _Files: pdr.read_prop_vec("_Files"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPFile {
    pub _FileName: String,               // read_prop
    pub _Versions: Vec<CBNPFileVersion>, // read_struct_vec
}

impl pd::Readable for CBNPFile {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFile {
        CBNPFile {
            _FileName: pdr.read("_FileName"),
            _Versions: pdr.read("_Versions"),
        }
    }
}

#[derive(Debug)]
pub struct CBNPFileVersion {
    pub _VersionNumber: u32,
    pub _FileSize: u32,
    pub _7ZFileSize: u32,
    pub _FileTime: u32,
    pub _PatchSize: u32,
    pub _HashKey: Vec<u32>, // read_prop_vec
}

impl pd::Readable for CBNPFileVersion {
    fn read(pdr: &mut pd::PersistentDataRecord) -> CBNPFileVersion {
        CBNPFileVersion {
            _VersionNumber: pdr.read("_VersionNumber"),
            _FileSize: pdr.read("_FileSize"),
            _7ZFileSize: pdr.read("_7ZFileSize"),
            _FileTime: pdr.read("_FileTime"),
            _PatchSize: pdr.read("_PatchSize"),
            _HashKey: pdr.read_prop_vec("_HashKey"), // read_prop_vec
        }
    }
}
