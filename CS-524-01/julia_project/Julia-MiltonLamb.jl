#=
Julia-MiltonLamb.rb
Author: Milton Lamb
Date: 2025 November 28
CS-524 Fall 2025

Application: Course Registration System

Usage: julia Julia-MiltonLamb.rb

Description: This is a registration system that reads a file of student and course 
information and outputs a report of all students and courses in the system.

The application reads in a file called "register.txt" that contains three sections:
- students (id name)
- courses (crn name)
- enrollments (id crn)

The application then builds two dictionaries (one for students and one for courses)
from the file. The dictionaries use a `{String, Student}` and `{String, Course}` key-value
pairs, respectively. Each instance of `Student` holds a list of references to the
`Course` instances that the student is enrolled in. The reverse is also true.

The `Student` struct instances are built upon parsing the first (students) section of the file.
The `Course` struct instances are built upon parsing the second (courses) section of the file.
These references are built upon parsing the third (enrollments) section of the file.

After the local relational database is built, the application outputs a report
using the `to_string` functions which take take the `Dict{String, Student}` and 
`Dict{String, Course}` dictionaries as arguments.

During the parsing of the file, state is managed by an array of function pointers. 
Each function pointed at is responsible for processing a section of the file. 
Upon a blank line, the index of the function pointer array is incremented. Moving
file parsing into the next state.

=#

struct Student
    id::String
    name::String
    courses::Vector
end

Student(id::String, name::String) = Student(id, name, [])

struct Course
    crn::String
    name::String
    students::Vector
end

Course(crn::String, name::String) = Course(crn, name, [])

### LINE PARSERS ###

function parse_line_student(line::String)::Tuple{String, String}
    parts = split(strip(line), limit=2)
    id = parts[1]
    name = get(parts, 2, "")
    (id, name)
end

function parse_line_course(line::String)::Tuple{String, String}
    parts = split(strip(line))
    crn = parts[1]
    name = join(parts[2:end], " ")
    (crn, name)
end

function parse_line_enrollment(line::String)::Tuple{String, String}
    parts = split(strip(line))
    student_id = parts[1]
    course_id = parts[2]
    (student_id, course_id)
end

### TO STRING ###

function to_string(students::Dict{String, Student})::String
    join(map(to_string, values(students)), "\n")
end

function to_string(student::Student)::String
    course_list = join(map(c -> "  - $(c.crn) $(c.name)", student.courses), "\n")
    course_count = length(student.courses)
    
    result = "$(student.id) $(student.name) - $course_count course(s):\n"
    result *= isempty(course_list) ? "  (No courses)" : course_list
    result *= "\n\n"
end

function to_string(courses::Dict{String, Course})::String
    return join(map(to_string, values(courses)), "\n")
end

function to_string(course::Course)::String
    student_list = join(map(s -> "  - $(s.name)", course.students), "\n")
    student_count = length(course.students)
    
    result = "$(course.crn) $(course.name) - $student_count student(s):\n"
    result *= isempty(student_list) ? "  (No students)" : student_list
    result *= "\n\n"
end

### VALIDATION ###

function validate_enrollment(
    id::String, crn::String, students::Dict{String, Student}, courses::Dict{String, Course}, line::String
)::Bool
    has_student = haskey(students, id)
    has_course = haskey(courses, crn)
    
    if !has_student && !has_course
        println("WARNING | Unknown student <$id> and course <$crn> in line <$(strip(line))>. Skipping...")
    elseif !has_student
        println("WARNING | Unknown student <$id> in line <$(strip(line))>. Skipping...")
    elseif !has_course
        println("WARNING | Unknown course <$crn> in line <$(strip(line))>. Skipping...")
    end
    
    has_student && has_course
end

### MAIN ###

function main()::Nothing

    students = Dict{String, Student}()
    courses = Dict{String, Course}()

    function process_student!(line::String)::Nothing
        id, name = parse_line_student(line)
        students[id] = Student(id, name)
        nothing
    end

    function process_course!(line::String)::Nothing
        crn, name = parse_line_course(line)
        courses[crn] = Course(crn, name)
        nothing
    end

    function process_registration!(line::String)::Nothing
        id, crn = parse_line_enrollment(line)
        if validate_enrollment(id, crn, students, courses, line)
            push!(students[id].courses, courses[crn])
            push!(courses[crn].students, students[id])
        end
        nothing
    end

    process! = [process_student!, process_course!, process_registration!]
    i = 1

    print("\nParsing Register...\n\n")

    open("register.txt") do file
        for line in eachline(file)

            if isempty(strip(line))
                i += 1
            else
                process![i](line)
            end
        end
    end

    print("\n################ Students ################\n")
    print(to_string(students))
    print("\n\n################ Courses ################\n")
    print(to_string(courses))

    nothing

end

main()

#=
My opinion of Julia is that it is a perfectly adequate language. Julia aims to solve the
"two-language" problem, and does so by being mediocre at both. Julia is fine, it has all
the data structures you expect in a modern language like lists and dictionaries built in,
with reasonably good performance. There is nothing wholey unexpected in the syntax though
I had some issues trying to explicitly say the function returned nothing. Of course I
didn't have to do that, but that brings me to my next point.

This is vibe based, but the typing system/typing syntax feels weird to me. At first I
I thought it was because it was dynamic and strongly typed, but Python is that way as
well. Then I thought maybe it was because it was inferred and strong, but Rust does that
as well. I believe it is the contrast between Julia's goal of making you feel like
you are wrting in a language like Python, but in reality you are writing in a language
like C++. It's acceptable and I'm sure I'd get used to it, but I definitely experienced
cognitive dissonance.

I understand that a central theme of Julia is its mulitple dispatch system, which is 
basically function overloading that works a little different. I would say that the
feature is probably implemented in the best manner it could be, but I think it is an
inherently flawed foundation to build upon. I subscribe to the idea that procedure
overloading is, in general, harmful. I do think there is a place for operator overloading
when it helps a type behave as expected, but procedure overloading makes it more 
difficult to track program execution (readbaility) outside of debug mode.

I suppose it yields slightly improved writability, but in general I place much less 
importance on writability as long as the language isn't pushing it to extremes. A 
developer spends much less time writing code than they do reading it. It does allow
the developer to add orthogonality to their code, but in a fairly superficial way 
since a procedure still must be written for each case.

Overall, I have no problem with Julia, but I don't think it really solves a problem that
exists. I don't mind writing small programs in a simple language and more complicated
programs in a more complex language. Other languages handle procedure overloading just 
fine if you want that functionality. Other languages can write classes to behave as structs
if desired. Other languages have lists and dictionaries built in. Other languages can run
just as fast or faster. Finally, while I can develop in vim just fine, I prefer an IDE and
the advantages that come with it, but Julia is lacking crucial IDE extensions that perform
static analysis on code before attempting compilation. 

=#