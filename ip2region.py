import sys,ctypes
from ctypes import c_voidp,c_char_p,Structure, POINTER
import os


class Ip2RegionStruct(Structure): pass

class Ip2Region:
    def __init__(self,db_file):
        prefix = {'win32': ''}.get(sys.platform, 'lib')
        extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
        self.__lib = ctypes.cdll.LoadLibrary(prefix + "ip2region" + extension)
        self.__lib.free_region.argtypes=(c_voidp,)

        self.__lib.ip2region_new.restype = POINTER(Ip2RegionStruct)
        self.__lib.ip2region_new.argtypes=(c_char_p,)

        self.__lib.ip2region_get_region.argtypes= (POINTER(Ip2RegionStruct),c_char_p)
        self.__lib.ip2region_get_region.restype = c_voidp

        self.__lib.ip2region_free.argtypes=(POINTER(Ip2RegionStruct),)
        self.__ip2region_ptr = self.__lib.ip2region_new(db_file.encode("utf-8"))

    def get_region(self,ip):
        ptr = self.__lib.ip2region_get_region(self.__ip2region_ptr,ip.encode("utf-8"))
        try:
            value = ctypes.cast(ptr,c_char_p).value
            value = value if value is None else value.decode("utf-8")
            return value
        finally:
            self.__lib.free_region(ptr)

    def __del__(self):
        self.__lib.ip2region_free(self.__ip2region_ptr)

if __name__ == "__main__":
    db_file = os.path.join(os.path.abspath(os.path.dirname(__file__)),"ip2region.db")
    print(db_file)
    ip2region = Ip2Region(db_file)
    ip = input("ip2region>")
    while ip != "quit":
        print(ip2region.get_region(ip))
        ip = input("ip2region>")