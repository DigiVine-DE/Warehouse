#!/usr/bin/python3

import sys
import datetime
from pathlib import Path



def reformat_date(date_str):
  #10.05.2021  12:17:43.425 => 2021-05-10  12:17:43.425
  return datetime.datetime.strptime(date_str, '%d.%m.%Y %H:%M:%S.%f').strftime('%Y-%m-%d %H:%M:%S.%f')

if len(sys.argv) < 4:
  print("Missing argument(s). Expected <file> <field-name> <run-id>")
  sys.exit(-1)

field_name = sys.argv[2]
run_id = int(sys.argv[3])

input_path = Path(sys.argv[1])
if not input_path.is_file():
  print("File '"+sys.argv[1]+"' does not exist")
  sys.exit(-1)


i = 0
with input_path.open() as input_path:
  for line in input_path:
    i+=1
    import_str = f'{i}-{field_name}'
    tokens = line.split(";")
    timestamp = reformat_date(tokens[0])
    long = float(tokens[1])
    lat = float(tokens[2])
    height = float(tokens[9])
    deviation = float(tokens[12])
    speed = float(tokens[14])
    plant_distance = float(tokens[20])
    row_distance = float(tokens[21])
    row_number = float(tokens[30])

    sql_str = f'INSERT INTO public.plant(planting_run_id,import_key,timestamp,height,deviation,speed,plant_distance,row_distance,row_number,lat,lon) VALUES ({run_id},\'{import_str}\',\'{timestamp}\',{height},{deviation},{speed},{plant_distance},{row_distance},{row_number},{lat},{long});'
    print(sql_str)



