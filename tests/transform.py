# Function to convert a hex value to a 7-letter string
def convert_hex_to_string(hex_value):
    binary_value = bin(int(hex_value, 16))[2:].zfill(
        8
    )  # Convert to binary and pad to 8 bits
    result = ""

    for i in range(8):
        if i == 2:
            continue
        if binary_value[i] == "1":
            # result += "CZIDB-VN"[i]
            result += "NV-BDIZC"[i]
        else:
            result += "-"

    return result


# Input and output file names
input_file = "nestest_trace_original.txt"
output_file = "nestest_trace.txt"

# Read the input file and process each line
with open(input_file, "r") as infile, open(output_file, "w") as outfile:
    for line in infile:
        parts = line.split(" P:")  # Split the line at "P:" to isolate the hex value
        if len(parts) == 2:
            hex_value = parts[1][
                :2
            ]  # Extract the first two characters as the hex value
            modified_line = line.replace(
                f" P:{hex_value}", " P:[" + convert_hex_to_string(hex_value) + "]"
            )
            outfile.write(modified_line)
