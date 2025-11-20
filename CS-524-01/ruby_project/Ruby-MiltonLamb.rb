=begin

Ruby-MiltonLamb.rb
Author: Milton Lamb
Date: 2025 November 18
CS-524 Fall 2025

Application: Course Registration System

Usage: ruby Ruby-MiltonLamb.rb

Description: This is a registration system that reads a file of student and course 
information and outputs a report of all students and courses in the system.

The application reads in a file called "register.txt" that contains three sections:
- students (id name)
- courses (crn name)
- enrollments (id crn)

The application then builds two hashes (one for students and one for courses)
from the file. The hashes use a `(string, Student)` and `(string, Course)` key-value
pairs, respectively. Each instance of `Student` holds a list of references to the
`Course` instances that the student is enrolled in. The reverse is also true.

The `Student` instances are built upon parsing the first (students) section of the file.
The `Course` instances are built upon parsing the second (courses) section of the file.
These references are built upon parsing the third (enrollments) section of the file.

After the local relational database is built, the application outputs a report
using the `to_s` methods of the `Student` and `Course` instances.

During the parsing of the file, state is managed by an array of lambdas. Each lambda
is responsible for processing a section of the file. Upon a blank line, the index of 
the lambda array is incremented. Moving file parsing into the next state.

=end


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

=begin

I hadn't written anything in Ruby since I have become what I would call 
a 'proper' programmer with a wide range of experience. I didn't learn much
from writing the algorithm, but that is largely due to my range of experience
parsing and manupulating data. This project is fairly straight forward, as far
as real world data tends to go.

I definitely did learn Ruby syntax that I wasn't familar with having not tried
it out in over a decade. From what was needed for this project, Ruby seems to
work reasonably similar to Python and JavaScript. It highly flexible and forgiving
from a typing perspective which makes it excellent for prototyping and small 
projects, but I can't see myself ever wanting to use it for a large project. 
That may be my own biases though, as I prefer strongly typed languages for the 
speed and reliability they provide.

I found it dissappointing that a language that is touted as 'everything is an 
object' doesn't support first class functions, instead requiring some kind of 
wrapper to pass them around. This isn't bad, but I found it jarring from the 
way Ruby describes itself. This is a lack of orthogonality I expected to be present. 
I thought the use of '@' sigils was interesting and less verbose, more readable 
and writable than using `this` or `self`. Ruby's required naming conventions are 
interesting though I prefer the more modern method implemented by Rust and Python 
to use IDE functionality to enforce a particular convention. The implementation of
'true' global values is fascinating since most languages don't have truly global 
variables. They are most often namespace or module scoped. Even in JS which supports
them this is more of an side-effect of the language's design rather than intentional. 
JS (ES6) has even removed global variables as the default behavior.

The built-in reflection is always a nice feature for niche cases; however, the overhead
often makes it not worth it. This project didn't require the use of what seems to 
possibly be Ruby's most unique feature of being able to compactly generate domain 
specific languages using the `method_missing` method. This is a feature of Ruby 
I'd be interested in examining further in the future.

=end