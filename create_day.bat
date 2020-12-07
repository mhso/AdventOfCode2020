@ECHO OFF

SET /P day=Enter day: 

python create_day.py %day%

PAUSE