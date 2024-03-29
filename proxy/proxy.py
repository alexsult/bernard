#!/usr/bin/env python3

import hashlib
import time
from werkzeug.contrib.cache import FileSystemCache
import requests
from flask import Flask, request


app = Flask(__name__)

HOST_TO_REQUEST = 'musicbrainz.org'
CACHE = FileSystemCache("cache", threshold=0, default_timeout=0)


@app.route('/')
def root():
    req = requests.get('https://' + HOST_TO_REQUEST + '/')
    return req.content


@app.route('/<path:other>')
def other(other):
    url = "https://{}/{}{}".format(HOST_TO_REQUEST, other,
                                   ("?{}".format(
                                       request.query_string.decode('utf-8'))
                                    if request.query_string else ""))

    hashed_url = hashlib.sha224(url.encode('utf-8')).hexdigest()
    print(hashed_url)
    rv = CACHE.get(hashed_url)
    if rv is None:
        print("MISS")
        req = requests.get(url)
        CACHE.set(hashed_url, req.content)
        time.sleep(2)
        return req.content
    return rv


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)
