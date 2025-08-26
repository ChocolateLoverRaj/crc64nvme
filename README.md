A CLI that computes the CRC64-NVME checksum from stdin and outputs it to stdout.

For example, if you wanted to check the checksum of a file and compare it to the checksum on AWS:
```bash
cat file | crc64nvme | base64
```
