section .data
tape times 30000 db 0
section .text
global _start
_start:
lea rsi, [rel tape]
mov [rsi], 0
add byte [rsi], 1
cmp byte [rsi], 0
LS_1:
jz LE_1
add rsi, 1
cmp byte [rsi], 0
LS_2:
jz LE_2
sub rsi, 1
sub byte [rsi], 1
cmp byte [rsi], 0
LS_3:
jz LE_3
jmp LS_3
LE_3:
add rsi, 1
add byte [rsi], 1
cmp byte [rsi], 0
LS_4:
jz LE_4
add rsi, 1
add byte [rsi], 3
add rsi, 1
cmp byte [rsi], 0
LS_5:
jz LE_5
add byte [rsi], 11
add rsi, 1
jmp LS_5
LE_5:
cmp byte [rsi], 0
LS_6:
jz LE_6
add rsi, 1
jmp LS_6
LE_6:
sub byte [rsi], 1
cmp byte [rsi], 0
LS_7:
jz LE_7
sub rsi, 1
jmp LS_7
LE_7:
add rsi, 1
sub byte [rsi], 1
jmp LS_4
LE_4:
jmp LS_2
LE_2:
add byte [rsi], 10
sub rsi, 1
jmp LS_1
LE_1:
add rsi, 1
add rsi, 5
sub byte [rsi], 4
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 2
add byte [rsi], 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 1
sub byte [rsi], 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add byte [rsi], 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 1
sub byte [rsi], 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 2
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add byte [rsi], 3
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub byte [rsi], 6
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 1
sub byte [rsi], 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 2
add byte [rsi], 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
sub rsi, 1
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
mov rax, 60
xor rdi, rdi
syscall