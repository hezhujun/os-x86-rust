import os

LOADER_SECTOR_COUNT = 4
KERNEL_SECTOR_COUNT = 100

if __name__ == "__main__":
    loader_size = os.path.getsize("loader.bin")
    kernel_size = os.path.getsize("kernel.bin")
    if loader_size > LOADER_SECTOR_COUNT * 512:
        print("loader.bin size {} > {} * 512".format(loader_size, LOADER_SECTOR_COUNT))
        exit(-1)
    
    if kernel_size > KERNEL_SECTOR_COUNT * 512:
        print("kernel.bin size {} > {} * 512".format(kernel_size, KERNEL_SECTOR_COUNT))
        exit(-2)
