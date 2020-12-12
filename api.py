# Broadcasts info in json
import urllib.request
import os
from mpd import MPDClient
from flask import Flask, Response
from flask_cors import CORS



# Flask Stuff
api = Flask(__name__)
CORS(api)


@api.route('/status', methods=['GET'])
def getstatus():
    return Response("", mimetype='application/json')


if __name__ == '__main__':
    api.run(debug=True, host="localhost", port=9001)
