url = "http://aws.imd.gov.in:8091/AWS/temp.php?a=60&b=ALL_STATE"

import requests
import requests_cache
import json
import checkmsg_pb2
requests_cache.install_cache('url_cache')
# Step 1: Fetch the data from the URL
response = requests.get(url)
data = response.text

# Assuming the data is a list of strings enclosed in square brackets
# Remove the square brackets and split the string into a list of strings
data_list = data[1:-1].split('","')

# Step 2: Parse each string in the list

parsed_data = []
# with open("test", 'w', encoding='utf-8') as file:   
with open('datacontent.pb', 'wb') as f:     
	for item in data_list:

		# Split each string by commas to get the individual values
		values = (item.split(','))
		# file.write(item)
		# file.write("\n") # Write each dictionary on a new line
		values_len = len(item.split(','))
		# filtered_rows = [row for row in rows if row['lat'] is not None]

		# print(values)
		# Assuming the data structure is consistent, you can access each value by its index
		if values_len==9:
			latitude = values[0]
			longitude = values[1]
			aws = values[2]
			state = values[3]
			region = values[4]
			location = values[5]
			temperature = values[6]
			date = values[7]
			time = values[8]

			# Create a dictionary for each item for easier access
			# parsed_item = {
			#     "latitude": latitude,
			#     "longitude": longitude,
			#     "aws": aws,
			#     "state": state,
			#     "region": region,
			#     "location": location,
			#     "temperature": temperature,
			#     "date": date,
			#     "time": time
			# }
			# parsed_data.append(parsed_item)

			parsed_item = checkmsg_pb2.DataContent(
				latitude= latitude,
				longitude= longitude,
				aws= aws,
				state= state,
				region= region,
				location= location,
				temperature= temperature,
				date= date,
				time= time
			)
			f.write(parsed_item.SerializeToString())
# Open a file for writing
# with open('parsed_data.json', 'w') as file:
#     # Use json.dump to write the data to the file
#     json.dump(parsed_data, file, indent=4)
# Now parsed_data contains a list of dictionaries, each representing one item from the original data
print(parsed_data)