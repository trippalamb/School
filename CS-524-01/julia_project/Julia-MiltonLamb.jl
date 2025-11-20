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