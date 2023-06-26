; ModuleID = 'main_module'
source_filename = "main_module"

@string = global [12 x i8] c"Hello world\0A"

define i32 @main() {
entry:
  call void @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @string, i32 0, i32 0))
  ret i32 0
}

declare void @printf(i8*)
