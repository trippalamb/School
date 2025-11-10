# Represents a course in the registration system.
class Course

  # @return [String]
  attr_reader :name, :crn
  
  # Standard Constructor
  #
  # @param name [String] the name of the course
  # @param crn [String] the course reference number
  def initialize(name, crn)
    @name = name
    @crn = crn
    @students = []
  end

  # Adds a student to this course's enrollment list.
  #
  # @param student [Student] the student to enroll in this course
  # @return [Array<Student>] the updated list of enrolled students
  def add_student(student)
    @students << student
  end

  # Standard to string method
  #
  # @return [String] formatted course information with student list
  def to_s
    student_list = @students.map { |student| "  - #{student.name}" }.join("\n")
    student_count = @students.length
    
    result = "#{@crn} #{@name} - #{student_count} student(s):\n"
    result += student_list.empty? ? "  (No students)" : student_list
    result += "\n\n"
    result
  end
end

# Represents a student in the registration system.
class Student

  # @return [String]
  attr_reader :name, :id
  
  # Standard Constructor
  #
  # @param name [String] the student's name
  # @param id [String] the student's ID number
  def initialize(name, id)
    @name = name
    @id = id
    @courses = []
  end
  
  # Adds a course to this student's course list.
  #
  # @param course [Course] the course to add to the student's schedule
  # @return [Array<Course>] the updated list of courses
  def add_course(course)
    @courses << course
  end

  # Standard to string method
  #
  # @return [String] formatted student information with course list
  def to_s
    course_list = @courses.map { |course| "  - #{course.name}" }.join("\n")
    course_count = @courses.length
    
    result = "#{@name} (#{@id}) - #{course_count} course(s):\n"
    result += course_list.empty? ? "  (No courses)" : course_list
    result += "\n\n"
    result
  end
end

# Parses a student line from the registration file.
#
# Expected format: "<id> <name>"
# The name may contain spaces and continues to the end of the line.
#
# @param line [String] the line to parse
# @return [Array<String>] [<id>, <name>]
def parse_student_line(line)
  parts = line.strip.split(nil, 2)
  id = parts[0]
  name = parts[1] || ""
  [id, name]
end

# Parses a course line from the registration file.
#
# Expected format: "<crn> <name>"
# The <name> may contain multiple words.
#
# @param line [String] the line to parse
# @return [Array<String>] [<crn>, <name>]
def parse_course_line(line)
  parts = line.strip.split
  crn = parts[0]
  name = parts[1..-1].join(" ")
  [crn, name]
end

# Parses an enrollment line from the registration file.
#
# Expected format: "<id> <crn>"
#
# @param line [String] the line to parse
# @return [Array<String>] [<id>, <crn>]
# @raise [RuntimeError] if the line contains more than 2 parts
def parse_enrollment_line(line)
  parts = line.strip.split
  id = parts[0]
  crn = parts[1]
  if parts.length > 2
    raise "Expected <2> parts, found <#{parts.length}> in line <#{line}>."
  end
  [id, crn]
end

# Validates that both student and course exist before enrollment.
#
# Prints warning messages to stdout if either the student ID or CRN
# is not found in the respective hash tables.
#
# @param id [String] the student ID to validate
# @param crn [String] the course CRN to validate
# @param students [Hash<String, Student>] hash of all students by ID
# @param courses [Hash<String, Course>] hash of all courses by CRN
# @return [Boolean] true if both student and course exist, false otherwise
def validate_enrollment(id, crn, students, courses, line)
  has_student = students.has_key?(id)
  has_course = courses.has_key?(crn)

  if !has_student && !has_course
    puts "WARNING | Unknown student <#{id}> and course <#{crn}> in line <#{line.chomp()}>. Skipping..."
  elsif !has_student
    puts "WARNING | Unknown student <#{id}> in line <#{line.chomp()}>. Skipping..."
  elsif !has_course
    puts "WARNING | Unknown course <#{crn}> in line <#{line.chomp()}>. Skipping..."
  end
  
  has_student && has_course
end

# Processes a student line and adds the student to the students hash.
#
# @param line [String] a line containing student information
# @param students [Hash<String, Student>] hash to store the new student in
def process_student(line, students)
  id, name = parse_student_line(line)
  students[id] = Student.new(name, id)
end

# Processes a course line and adds the course to the courses hash.
#
# @param line [String] a line containing course information
# @param courses [Hash<String, Course>] hash to store the new course in
def process_course(line, courses)
  crn, name = parse_course_line(line)
  courses[crn] = Course.new(name, crn)
end

# Processes an enrollment line and links student to course.
#
# @param line [String] the line containing enrollment information
# @param students [Hash<String, Student>] hash of all students
# @param courses [Hash<String, Course>] hash of all courses
def process_enrollment(line, students, courses)
  id, crn = parse_enrollment_line(line)
  is_valid = validate_enrollment(id, crn, students, courses, line)
  students[id].add_course(courses[crn]) if is_valid
  courses[crn].add_student(students[id]) if is_valid
end


# Processes register.txt with three blank-line-separated sections:
# students, courses, enrollments.
#
# @raise [RuntimeError] if more than 3 sections found
# @raise [Errno::ENOENT] if register.txt not found
def parse_register(process, students, courses)

  # Current section index (0=students, 1=courses, 2=enrollments)
  i = 0
  puts "\nParsing Register...\n\n"
  File.open("register.txt", "r") do |file|

    file.each_line do |line|

      if line.strip.empty?
        i += 1
        if i > process.length
          raise "Expected <#{process.length}> sections, found at least <#{i + 1}>."
        end
      else
        process[i].call(line)
      end

    end
  end

end

# Main entry point to program
def main

  students = {}
  courses = {}

  # Array of functions for processing each section of the file
  process = [
    ->(line) { process_student(line, students) },
    ->(line) { process_course(line, courses) },
    ->(line) { process_enrollment(line, students, courses) }
  ]

  parse_register(process, students, courses)
  puts "\n################ Students ################\n"
  puts students.values
  puts "\n################ Courses ################\n"
  puts courses.values

  
end

main()