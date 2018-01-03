#!/usr/bin/env python3 

from flask import Flask
from flask import request
 
import requests
 
app = Flask(__name__)
 
hosttorequest = 'musicbrainz.org'
 
@app.route('/')
def root():
    r = requests.get('https://'+hosttorequest+'/')
    print(r.content)
    return r.content
 
@app.route('/<path:other>')
def other(other):
    print("Q {}".format(request.query_string));
    url = "https://{}/{}{}".format(hosttorequest, other,
            ("?{}".format(request.query_string.decode('utf-8')) if request.query_string else ""))
    print("Other: {}".format(url))
    r = requests.get(url)
    print(r.content)
    return r.content
     
if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)
