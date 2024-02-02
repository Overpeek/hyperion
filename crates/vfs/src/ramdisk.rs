use alloc::{
    boxed::Box,
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::any::Any;

use hyperion_mem::pmm::{PageFrame, PFA};
use lock_api::Mutex;

use crate::{
    device::{DirectoryDevice, FileDevice},
    error::{IoError, IoResult},
    tree::{DirRef, FileRef, Node, WeakDirRef},
    AnyMutex,
};

//

pub struct File {
    // bytes: Vec<u8>,
    pages: Vec<PageFrame>,
    len: usize,
}

impl File {
    pub fn new(bytes: &[u8]) -> Self {
        let pages = bytes.len().div_ceil(0x1000);
        let mut pages = PFA.alloc(pages);
        pages.as_bytes_mut()[..bytes.len()].copy_from_slice(bytes);

        Self {
            pages: vec![pages],
            len: bytes.len(),
        }
    }

    pub fn new_empty<Mut: AnyMutex>() -> FileRef<Mut> {
        Arc::new(Mutex::new(Self {
            pages: Vec::new(),
            len: 0,
        })) as _
    }
}

pub struct StaticRoFile {
    bytes: &'static [u8],
}

impl StaticRoFile {
    pub const fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

pub struct Directory<Mut: AnyMutex> {
    pub name: Arc<str>,
    pub children: BTreeMap<Arc<str>, Node<Mut>>,
    pub parent: Option<WeakDirRef<Mut>>,

    nodes_cache: Option<Arc<[Arc<str>]>>,
}

//

impl FileDevice for File {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, len: usize) -> IoResult<()> {
        self.len = len;
        Ok(())
    }

    fn map_phys(&mut self, min_bytes: usize) -> IoResult<Box<[PageFrame]>> {
        let mut pages_left = min_bytes.div_ceil(0x1000);
        let pages = self
            .pages
            .iter()
            .filter_map(move |page| {
                if pages_left == 0 {
                    return None;
                }

                let n = pages_left.min(page.len());
                pages_left = pages_left.saturating_sub(page.len());

                Some(unsafe { PageFrame::new(page.physical_addr(), n) })
            })
            .collect::<Box<[PageFrame]>>();

        Ok(pages)
    }

    fn unmap_phys(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn read(&self, offset: usize, mut buf: &mut [u8]) -> IoResult<usize> {
        let initial_len = buf.len();

        let mut current_page_start = 0usize;
        let mut pages = self.pages.iter();
        while !buf.is_empty() {
            let Some(at) = pages.next() else {
                return Ok(initial_len - buf.len());
            };

            if let Some(read_from) = current_page_start
                .checked_sub(offset)
                .and_then(|read_from| at.as_bytes().get(read_from..))
            {
                let read_limit = read_from.len().min(buf.len());
                buf[..read_limit].copy_from_slice(&read_from[..read_limit]);
                buf = buf.split_at_mut(read_limit).1;
            }

            current_page_start += at.byte_len();
        }

        Ok(initial_len)
    }

    fn write(&mut self, offset: usize, mut buf: &[u8]) -> IoResult<usize> {
        self.len = self.len.max(offset + buf.len());

        let initial_len = buf.len();

        let mut current_page_start = 0usize;
        let mut pages = self.pages.iter_mut();
        while !buf.is_empty() {
            let Some(at) = pages.next() else {
                let pages = (offset + buf.len() - current_page_start).div_ceil(0x1000);
                let mut pages = PFA.alloc(pages);
                pages.as_bytes_mut().fill(0);
                pages.as_bytes_mut()[..buf.len()].copy_from_slice(buf);
                self.pages.push(pages);
                break;
            };

            if let Some(write_to) = current_page_start
                .checked_sub(offset)
                .and_then(|write_to| at.as_bytes_mut().get_mut(write_to..))
            {
                let write_limit = write_to.len().min(buf.len());
                write_to[..write_limit].copy_from_slice(&buf[..write_limit]);
                buf = buf.split_at(write_limit).1;
            }

            current_page_start += at.byte_len();
        }

        Ok(initial_len)
    }
}

impl FileDevice for StaticRoFile {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn set_len(&mut self, _: usize) -> IoResult<()> {
        Err(IoError::PermissionDenied)
    }

    fn read(&self, offset: usize, buf: &mut [u8]) -> IoResult<usize> {
        self.bytes.read(offset, buf)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> IoResult<usize> {
        Err(IoError::PermissionDenied)
    }
}

impl<Mut: AnyMutex> DirectoryDevice<Mut> for Directory<Mut> {
    fn get_node(&mut self, name: &str) -> IoResult<Node<Mut>> {
        if let Some(node) = self.children.get(name) {
            Ok(node.clone())
        } else {
            Err(IoError::NotFound)
        }
    }

    fn create_node(&mut self, name: &str, node: Node<Mut>) -> IoResult<()> {
        match self.children.entry(name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(node);
                self.nodes_cache = None;
                Ok(())
            }
            Entry::Occupied(_) => Err(IoError::AlreadyExists),
        }
    }

    fn nodes(&mut self) -> IoResult<Arc<[Arc<str>]>> {
        Ok(self
            .nodes_cache
            .get_or_insert_with(|| self.children.keys().cloned().collect())
            .clone())
    }
}

impl<Mut: AnyMutex> Directory<Mut> {
    pub fn new(name: impl Into<Arc<str>>) -> Self {
        Self {
            name: name.into(),
            children: BTreeMap::new(),
            parent: None,

            nodes_cache: None,
        }
    }

    pub fn new_ref(name: impl Into<Arc<str>>) -> DirRef<Mut> {
        Arc::new(Mutex::new(Self::new(name))) as _
    }
}
