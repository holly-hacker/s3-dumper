# s3-dumper
This is a tool to dump exposed S3-compatible buckets.

Please note that this is a tool I developed for personal usage, so it may not be ideal for you or
have the features you need. It may be buggy in ways I don't care about or it may straight up not
work for you.

## Basic usage
```sh
s3-dumper list-files https://storage.example.com/
s3-dumper --prefix "images/" download https://storage.example.com/ downloaded-images
```

## Motivation
There are many tools out there that can dump s3 buckets but they all seem to require the bucket
name (which is not always known) or credentials (which I don't have). This tool only requires a url
where the bucket is exposed on. If you can see the XML file listing, you can download it!
