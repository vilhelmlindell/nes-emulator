trace_path = "trace.txt"
nestest_trace_path = "tests/nestest_trace.txt"

previous_trace_line = ""
previous_nestest_trace_line = ""


def extract_text(input_string, string_to_find):
    # Find the index of the string to find within the input string
    start_index = input_string.find(string_to_find)

    if start_index != -1:
        # Find the next space character from the start index
        next_space_index = input_string.find(" ", start_index)
        if next_space_index == -1:
            next_space_index = input_string.find("\n", start_index)

        if next_space_index != -1:
            # Extract the substring from the start index to the next space
            extracted_text = input_string[
                start_index + len(string_to_find) : next_space_index
            ]
            return extracted_text
    return None  # Return None if the string to find or space is not found


strings_to_find = ["A:", "X:", "Y:", "P:", "SP:", "CYC:"]

with open(trace_path, "r") as trace_file, open(
    nestest_trace_path, "r"
) as nestest_trace_file:
    for line_index, (trace_line, nestest_trace_line) in enumerate(
        zip(trace_file, nestest_trace_file), start=1
    ):
        mismatch_found = False
        string_mismatch = ""

        if trace_line[:19] != nestest_trace_line[:19]:
            mismatch_found = True

        for string_to_find in strings_to_find:
            if extract_text(trace_line, string_to_find) != extract_text(
                nestest_trace_line, string_to_find
            ):
                mismatch_found = True

        if mismatch_found:
            print(f"Mismatch at line {line_index}:")
            if string_mismatch != "":
                print(string_mismatch)
            break

        previous_trace_line = trace_line
        previous_nestest_trace_line = nestest_trace_line
