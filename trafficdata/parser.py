from typing import Callable, Tuple, Iterable, Mapping
from collections.abc import Iterator
from dataclasses import dataclass
from pathlib import PosixPath, Path
import struct

import numpy as np

@dataclass
class MinuteData:

    date: str

    weekday: int
    hour: int
    week_hour: int
    minute: int
    week_minute: int
    day_minute: int

    has_dns_data: int
    mattermost: int
    reddit: int
    youtube: int
    twitch: int

    is_at_home: int
    phone_seen: int
    mac_seen: int
    work_laptop_at_home: int

    traffic_volume: int

def bit_at(buf: int, pos: int) -> int:
    return (buf >> pos) & 1

class Column:

    Type = Callable[[int], int]

    @staticmethod
    def weekday(buf: int) -> int:
        return 0b111 & (buf >> 29)
    
    @staticmethod
    def hour(buf: int) -> int:
        return 0b11111 & (buf >> 24)
    
    @staticmethod
    def minute(buf: int) -> int:
        return 0b111111 & (buf >> 18)

    @staticmethod
    def bit_at(pos: int) -> Type:
        def lookup(buf: int) -> int:
            return (buf >> pos) & 1
        return lookup

    has_dns_data=bit_at(12)
    mattermost=bit_at(11)
    reddit=bit_at(10)
    youtube=bit_at(9)
    twitch=bit_at(8)
    is_at_home=bit_at(7)
    phone_seen=bit_at(6)
    mac_seen=bit_at(5)
    work_laptop_at_home=bit_at(4)

    @staticmethod
    def log_bytes(buf: int) -> int:
        return buf & 0xF

def read_columns(
    paths: Iterable[PosixPath],
    columns: Iterable[Tuple[str, Column.Type]],
    file_idx_col: str|None = None,
) -> Mapping[str, np.ndarray]:
    # get the number of rows
    size = 0
    for path in paths:
        size += path.stat().st_size // 4

    # allocate arrays
    arrays = [ np.empty(size, dtype=np.uint) for _ in columns ]
    if file_idx_col:
        file_arr = np.empty(size, dtype=np.uint)

    row = 0
    for file_idx, path in enumerate(paths):
        file_data = path.read_bytes()
        if file_idx_col:
            file_arr[row:row + (len(file_data) // 4)] = file_idx

        for (buf, ) in struct.iter_unpack(">L", file_data):
            for col_idx, col in enumerate(columns):
                arrays[col_idx][row] = col[1](buf)
            row += 1
    cols = {}
    if file_idx_col:
        cols[file_idx_col] = file_arr
    for i, (name, _) in enumerate(columns):
        cols[name] = arrays[i]
    return cols

def read_file(path: PosixPath|str) -> Iterator[MinuteData]:
    """
    this read_file method is very poorly optimized, but exposes all the data availible super easily
    """
    if type(path) == str:
        path = Path(path)
    data = path.read_bytes()
    for (buf, ) in struct.iter_unpack(">L", data):
        weekday = 0b111 & (buf >> (32 -  3))
        hour = 0b11111 & (buf >> (32 -  8))
        minute = 0b111111 & (buf >> (32 - 14))
        week_hour=(24 * weekday) + hour
        yield MinuteData(
            date=path.name,
            weekday=weekday,
            hour=hour,
            week_hour=week_hour,
            minute=minute,
            day_minute=hour * 60 + minute,
            week_minute=week_hour * 60 + minute,
            has_dns_data=bit_at(buf, 12),
            mattermost=bit_at(buf, 11),
            reddit=bit_at(buf, 10),
            youtube=bit_at(buf, 9),
            twitch=bit_at(buf, 8),
            is_at_home=bit_at(buf, 7),
            phone_seen=bit_at(buf, 6),
            mac_seen=bit_at(buf, 5),
            work_laptop_at_home=bit_at(buf, 4),
            traffic_volume=buf & 0xF,
        )


@dataclass
class LowResData:
    
    time: int
    tags: int
    value: int

def read_file_low_res(path: PosixPath):
    data = path.read_bytes()
    for (buf, ) in struct.iter_unpack(">I", data):
        yield LowResData(
            time = 0b11111111111111 & (buf >> 18),
            tags = 0b111111111 & (buf >> 8),
            value = buf & 0xF,
        )
