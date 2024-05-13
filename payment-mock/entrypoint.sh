#!/bin/bash

# Start App
gunicorn -c gunicorn_config.py payment_api.wsgi:application
