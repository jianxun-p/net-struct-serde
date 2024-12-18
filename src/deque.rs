use core::mem::MaybeUninit;

pub trait Deque<T: Sized + Clone> {
    type Ok;
    type Error;
    fn size(&self) -> usize;
    fn capacity(&self) -> usize;
    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;
    fn push_front(&mut self, v: &T) -> Result<Self::Ok, Self::Error>;
    fn push_back(&mut self, v: &T) -> Result<Self::Ok, Self::Error>;
    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;
    fn take<const N: usize>(&mut self) -> Option<[T; N]>;
}

#[derive(Debug, Clone)]
pub struct FixedSizeDeque<T: Sized + Clone, const N: usize> {
    buf: [MaybeUninit<T>; N],
    start: usize,
    size: usize,
}

pub struct FixedSizeDequeIter<T: Sized + Clone, const N: usize> {
    pub(self) deque: &FixedSizeDeque<T, N>,
    pub(self) current: usize,
}

impl<T: Sized + Clone, const N: usize> FixedSizeDeque<T, N> {
    pub fn new() {
        Self {
            buf: [MaybeUninit::<T>::uninit(); N],
            start: 0,
            size: 0,
        }
    }

    pub fn iter(&self) -> FixedSizeDequeIter<T, N> {
        FixedSizeDequeIter {
            deque: &self,
            current: 0,
        }
    }

    pub fn to_vec(self) -> heapless::Vec<T, N> {
        let mut v = heapless::Vec::new();
        unsafe {
            self.iter().for_each(|i| v.push_unchecked(i));
        }
        v
    }
}

impl<T: Sized + Clone, const N: usize> Deque<T> for FixedSizeDeque<T, N> {
    type Ok = ();

    type Error = ();

    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    #[inline]
    fn capacity(&self) -> usize {
        N
    }

    #[inline]
    fn front(&self) -> Option<&T> {
        match self.size > 0 {
            true => unsafe { Some(&self.buf[self.start].assume_init()) },
            false => None,
        }
    }

    #[inline]
    fn back(&self) -> Option<&T> {
        match self.size > 0 {
            true => unsafe { Some(&self.buf[self.start + self.size - 1].assume_init()) },
            false => None,
        }
    }

    #[inline]
    fn push_front(&mut self, v: &T) -> Result<Self::Ok, Self::Error> {
        match self.size + 1 < N {
            true => unsafe {
                self.start = (self.start + N - 1) % N;
                self.size += 1;
                self.buf[self.start].write(v.clone());
                Ok(())
            },
            false => Err(()),
        }
    }

    #[inline]
    fn push_back(&mut self, v: &T) -> Result<Self::Ok, Self::Error> {
        match self.size + 1 < N {
            true => unsafe {
                self.size += 1;
                self.buf[(self.start + self.size - 1) % N].write(v.clone());
                Ok(())
            },
            false => Err(()),
        }
    }

    #[inline]
    fn pop_front(&mut self) -> Option<T> {
        match self.size > 0 {
            true => unsafe {
                let old_start = self.start;
                self.start = (self.start + 1) % N;
                self.size -= 1;
                Some(self.buf[old_start].assume_init())
            },
            false => None,
        }
    }

    #[inline]
    fn pop_back(&mut self) -> Option<T> {
        match self.size > 0 {
            true => unsafe {
                self.size -= 1;
                Some(self.buf[(self.start + self.size) % N].assume_init())
            },
            false => None,
        }
    }

    #[inline]
    fn take<const S: usize>(&mut self) -> Option<[T; S]> {
        if self.size < N {
            return None;
        }
        [MaybeUninit::uninit(); S]
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = self.buf[(self.start + i) % N])
            .map(|v| unsafe { v.assume_init() })
    }
}

impl<T: Sized + Clone, const N: usize> Iterator for FixedSizeDequeIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current < self.deque.size {
            true => unsafe {
                self.current += 1;
                Some(self.deque.buf[self.deque.start + self.current - 1].assume_init())
            },
            false => None,
        }
    }
}
