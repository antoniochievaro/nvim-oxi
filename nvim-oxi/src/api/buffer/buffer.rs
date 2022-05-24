use std::fmt;

use nvim_types::error::{ConversionError, Error as NvimError};
use nvim_types::{Array, BufHandle, Dictionary, Integer, NvimString, Object};

use super::opts::*;
use super::r#extern::*;
use crate::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

impl<H: Into<BufHandle>> From<H> for Buffer {
    fn from(handle: H) -> Self {
        Buffer(handle.into())
    }
}

impl Buffer {
    /// Binding to `nvim_buf_attach`.
    pub fn attach(
        &self,
        send_buffer: bool,
        opts: BufAttachOpts,
    ) -> Result<()> {
        todo!()
    }

    /// Binding to `nvim_buf_call`.
    pub fn call<F: FnMut() + 'static>(&self, mut fun: F) -> Result<()> {
        // let lua = mlua::Lua::new();

        // let nvim_buf_call = lua
        //     .globals()
        //     .get::<_, mlua::Table>("vim")
        //     .unwrap()
        //     .get::<_, mlua::Table>("api")
        //     .unwrap()
        //     .get::<_, mlua::Function>("nvim_buf_call")
        //     .unwrap();

        // let fun = lua.create_function_mut(move |_lua, ()| Ok(fun())).unwrap();
        // nvim_buf_call.call::<_, ()>((self.0, fun)).unwrap();

        Ok(())
    }

    // create_user_command

    /// Binding to `nvim_buf_del_keymap`.
    ///
    /// Unmaps a buffer-local mapping for the given mode.
    pub fn del_keymap(&mut self, mode: &str, lhs: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_del_keymap(0, self.0, mode.into(), lhs.into(), &mut err)
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_mark`.
    ///
    /// Deletes a named mark in the buffer. If the mark is not set in the
    /// buffer it will return false.
    pub fn del_mark(&mut self, name: &str) -> Result<bool> {
        let mut err = NvimError::default();
        let mark_was_deleted =
            unsafe { nvim_buf_del_mark(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| mark_was_deleted)
    }

    /// Binding to `nvim_buf_del_user_command`.
    pub fn nvim_buf_del_user_command(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_user_command(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_var`.
    ///
    /// Removes a buffer-scoped (b:) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_var(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    // Binding to `nvim_buf_delete`.
    pub fn delete(self, force: bool, unload: bool) -> Result<()> {
        todo!()
    }

    /// Binding to `nvim_buf_get_changedtick`.
    pub fn get_changedtick(&self) -> Result<usize> {
        let mut err = NvimError::default();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        err.into_err_or_else(|| ct.try_into().expect("always positive"))
    }

    // get_commands

    // get_keymap

    /// Binding to `nvim_buf_get_lines`.
    pub fn get_lines(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Vec<NvimString>> {
        todo!()
    }

    /// Binding to `nvim_buf_get_mark`.
    ///
    /// Returns a tuple `(row, col)` representing the position of the named
    /// mark. Marks are (1,0)-indexed.
    pub fn get_mark(&self, name: &str) -> Result<(usize, usize)> {
        todo!()
    }

    /// Binding to `nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer, replacing all invalid UTF-8
    /// byte sequences in the path with `U+FFFD REPLACEMENT CHARACTER` (�).
    pub fn get_name(&self) -> String {
        unsafe { nvim_buf_get_name(self.0, &mut NvimError::default()) }
            .to_string_lossy()
            .into_owned()
    }

    /// Binding to `nvim_buf_get_offset`.
    ///
    /// Returns the byte offset of a line (0-indexed, so line 1 has index 0).
    pub fn get_offset<Index>(&self, index: Index) -> Result<usize>
    where
        Index: Into<Integer>,
    {
        let mut err = NvimError::default();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index.into(), &mut err) };
        err.into_err_or_else(|| offset.try_into().expect("offset is positive"))
    }

    /// Binding to `nvim_buf_get_option`.
    ///
    /// Gets a buffer option value. Fails if the returned object couldn't be
    /// converted into the specified type.
    pub fn get_option<Value>(&self, name: &str) -> Result<Value>
    where
        Value: TryFrom<Object, Error = ConversionError>,
    {
        let mut err = NvimError::default();

        let obj =
            unsafe { nvim_buf_get_option(self.0, name.into(), &mut err) };

        // TODO: rewrite this as
        //
        // err.into_err_or_else(|| obj.try_into().map_err(crate::Error::from))
        //     .flatten()
        //
        // after https://github.com/rust-lang/rust/issues/70142 becomes stable.

        err.into_err_or_else(|| ())
            .and_then(|_| obj.try_into().map_err(crate::Error::from))
    }

    /// Bindint to `nvim_buf_get_text`.
    ///
    /// Gets a range from the buffer. Indexing is zero-based, with both row and
    /// column indices being end-exclusive.
    pub fn get_text<Int>(
        &self,
        start_row: Int,
        start_col: Int,
        end_row: Int,
        end_col: Int,
    ) -> Result<Vec<NvimString>>
    where
        Int: Into<Integer>,
    {
        todo!()
    }

    /// Binding to `nvim_buf_get_var`.
    ///
    /// Gets a buffer-scoped (b:) variable. Fails in the returned object
    /// couldn't be converted into the specified type.
    pub fn get_var<Value>(&self, name: &str) -> Result<Value>
    where
        Value: TryFrom<Object, Error = ConversionError>,
    {
        let mut err = NvimError::default();

        let obj = unsafe { nvim_buf_get_var(self.0, name.into(), &mut err) };

        // TODO: rewrite this as
        //
        // err.into_err_or_else(|| obj.try_into().map_err(crate::Error::from))
        //     .flatten()
        //
        // after https://github.com/rust-lang/rust/issues/70142 becomes stable.

        err.into_err_or_else(|| ())
            .and_then(|_| obj.try_into().map_err(crate::Error::from))
    }

    /// Binding to `nvim_buf_is_loaded`.
    ///
    /// Checks if a buffer is valid and loaded.
    pub fn is_loaded(&self) -> bool {
        unsafe { nvim_buf_is_loaded(self.0) }
    }

    /// Binding to `nvim_buf_is_valid`.
    ///
    /// Checks if a buffer is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_buf_is_valid(self.0) }
    }

    /// Binding to `nvim_buf_line_count`.
    ///
    /// Returns the number of lines in the given buffer.
    pub fn line_count(&self) -> Result<usize> {
        let mut err = NvimError::default();
        let count = unsafe { nvim_buf_line_count(self.0, &mut err) };
        err.into_err_or_else(|| count.try_into().expect("always positive"))
    }

    // set_keymap

    /// Binding to `nvim_buf_set_lines`.
    ///
    /// Sets (replaces) a line-range in the buffer. Indexing is zero-based,
    /// end-exclusive.
    pub fn set_lines<Int, Line, Lines>(
        &mut self,
        start: Int,
        end: Int,
        strict_indexing: bool,
        replacement: Lines,
    ) -> Result<()>
    where
        Int: Into<Integer>,
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_lines(
                0,
                self.0,
                start.into(),
                end.into(),
                strict_indexing,
                replacement
                    .into_iter()
                    .map(|line| line.into())
                    .collect::<Array>(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_mark`.
    ///
    /// Sets a named mark in the buffer. Marks are (1,0)-indexed, and passing 0
    /// as `line` deletes the mark.
    pub fn set_mark<Int>(
        &mut self,
        name: &str,
        line: Int,
        col: Int,
    ) -> Result<bool>
    where
        Int: Into<Integer>,
    {
        let mut err = NvimError::default();
        let mark_was_set = unsafe {
            nvim_buf_set_mark(
                self.0,
                name.into(),
                line.into(),
                col.into(),
                Dictionary::new(),
                &mut err,
            )
        };
        err.into_err_or_else(|| mark_was_set)
    }

    /// Binding to `nvim_buf_set_name`.
    ///
    /// Sets the full file name for a buffer.
    pub fn set_name<Name>(&mut self, name: Name) -> Result<()>
    where
        Name: Into<NvimString>,
    {
        let mut err = NvimError::default();
        unsafe { nvim_buf_set_name(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_option`.
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    pub fn set_option<Value>(&mut self, name: &str, value: Value) -> Result<()>
    where
        Value: Into<Object>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_option(0, self.0, name.into(), value.into(), &mut err)
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_text`.
    ///
    /// Sets (replaces) a range in the buffer. Indexing is zero-based, with
    /// both row and column indices being end-exclusive.
    pub fn set_text<Int, Line, Lines>(
        &mut self,
        start_row: Int,
        start_col: Int,
        end_row: Int,
        end_col: Int,
        replacement: Lines,
    ) -> Result<()>
    where
        Int: Into<Integer>,
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_text(
                0,
                self.0,
                start_row.into(),
                start_col.into(),
                end_row.into(),
                end_col.into(),
                replacement
                    .into_iter()
                    .map(|line| line.into())
                    .collect::<Array>(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_var`.
    ///
    /// Sets a buffer-scoped (b:) variable.
    pub fn set_var<Value>(&mut self, name: &str, value: Value) -> Result<()>
    where
        Value: Into<Object>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_var(self.0, name.into(), value.into(), &mut err)
        };
        err.into_err_or_else(|| ())
    }
}