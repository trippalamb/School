from segment_helpers import add_line_segment, draw_line_segments

# =============================================
#  ADD YOUR LINE SEGMENTS HERE!
# =============================================

add_line_segment(length=6, label="6 cm", color="blue")
add_line_segment(length=3, label="3 inches", color="red")
add_line_segment(length=18, label="banana", color="red")
add_line_segment(length=220, label="22 inches", color="magenta")

# =============================================
#  This line does the drawing — just run the file!
# =============================================
draw_line_segments()


#TODO: needs to account for negative and support more margin in height, for big numbers, it should also throw a helpful error message if the size or label are too long, and if the color isn't valid
