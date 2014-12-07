
use std::fmt::{Show, Error, Formatter};
use time::Timespec;
use libc;

use common::{MpdResult, FromConn};
use connection::{mpd_connection, MpdConnection};
use songs::MpdSongs;

#[repr(C)] struct mpd_playlist;

#[link(name = "mpdclient")]
extern "C" {
    fn mpd_playlist_dup(playlist: *const mpd_playlist) -> *mut mpd_playlist;
    fn mpd_recv_playlist(playlist: *mut mpd_connection) -> *mut mpd_playlist;
    fn mpd_playlist_free(playlist: *mut mpd_playlist);
    fn mpd_playlist_get_last_modified(playlist: *const mpd_playlist) -> libc::time_t;
    fn mpd_playlist_get_path(playlist: *const mpd_playlist) -> *const u8;

    fn mpd_send_list_playlists(connection: *mut mpd_connection) -> bool;
    fn mpd_send_list_playlist(connection: *mut mpd_connection, name: *const u8) -> bool;
}

pub struct MpdPlaylists<'a> {
    conn: *mut mpd_connection
}

impl<'a> FromConn for MpdPlaylists<'a> {
    fn from_conn<'a>(connection: *mut mpd_connection) -> Option<MpdPlaylists<'a>> {
        if unsafe { mpd_send_list_playlists(connection) } {
            Some(MpdPlaylists { conn: connection })
        } else {
            None
        }
    }
}

impl<'a> Iterator<MpdResult<MpdPlaylist>> for MpdPlaylists<'a> {
    fn next(&mut self) -> Option<MpdResult<MpdPlaylist>> {
        match FromConn::from_conn(self.conn) {
            Some(pl) => Some(Ok(pl)),
            None => match FromConn::from_conn(self.conn) {
                None => None,
                Some(e) => Some(Err(e))
            }
        }
    }
}

impl Drop for MpdPlaylist {
    fn drop(&mut self) {
        unsafe { mpd_playlist_free(self.pl) }
    }
}

impl Clone for MpdPlaylist {
    fn clone(&self) -> MpdPlaylist {
        let pl = unsafe { mpd_playlist_dup(self.pl as *const _) };
        if pl.is_null() {
            panic!("Out of memory!")
        }

        MpdPlaylist { pl: pl }
    }
}

impl Show for MpdPlaylist {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(f.write(b"MpdPlaylist { "));
        try!(f.write(b"path: "));
        try!(self.path().fmt(f));
        try!(f.write(b" }"));
        Ok(())
    }
}

pub struct MpdPlaylist {
    pl: *mut mpd_playlist
}

impl MpdPlaylist {
    pub fn path(&self) -> String {
        unsafe { String::from_raw_buf(mpd_playlist_get_path(self.pl as *const _)) }
    }

    pub fn last_mod(&self) -> Timespec { Timespec::new(unsafe { mpd_playlist_get_last_modified(self.pl as *const _) }, 0) }
}

impl FromConn for MpdPlaylist {
    fn from_conn(connection: *mut mpd_connection) -> Option<MpdPlaylist> {
        let pl = unsafe { mpd_recv_playlist(connection) };
        if pl.is_null() {
            None
        } else {
            Some(MpdPlaylist { pl: pl })
        }
    }
}

impl MpdPlaylist {
    pub fn songs<'a>(&self, conn: &'a mut MpdConnection) -> MpdResult<MpdSongs<'a>> {
        if unsafe { mpd_send_list_playlist(conn.conn, mpd_playlist_get_path(self.pl as *const _)) } {
            Ok(MpdSongs { conn: conn })
        } else {
            Err(FromConn::from_conn(conn.conn).unwrap())
        }
    }
}

