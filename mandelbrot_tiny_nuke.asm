section .data
tape times 30000 db 0
section .text
global _start
_start:
lea rsi, [rel tape]
mov [rsi], 0
add byte [rsi], 13
cmp byte [rsi], 0
LS_1:
jz LE_1
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 2
add rsi, 3
add byte [rsi], 5
add rsi, 1
add byte [rsi], 2
add rsi, 1
add byte [rsi], 1
sub rsi, 6
jmp LS_1
LE_1:
add rsi, 5
add byte [rsi], 6
add rsi, 1
sub byte [rsi], 3
add rsi, 10
add byte [rsi], 15
cmp byte [rsi], 0
LS_2:
jz LE_2
cmp byte [rsi], 0
LS_3:
jz LE_3
add rsi, 9
jmp LS_3
LE_3:
add byte [rsi], 1
cmp byte [rsi], 0
LS_4:
jz LE_4
sub rsi, 9
jmp LS_4
LE_4:
add rsi, 9
sub byte [rsi], 1
jmp LS_2
LE_2:
add byte [rsi], 1
cmp byte [rsi], 0
LS_5:
jz LE_5
add rsi, 8
cmp byte [rsi], 0
LS_6:
jz LE_6
sub byte [rsi], 1
jmp LS_6
LE_6:
add rsi, 1
jmp LS_5
LE_5:
sub rsi, 9
cmp byte [rsi], 0
LS_7:
jz LE_7
sub rsi, 9
jmp LS_7
LE_7:
add rsi, 8
cmp byte [rsi], 0
LS_8:
jz LE_8
sub byte [rsi], 1
jmp LS_8
LE_8:
add byte [rsi], 1
sub rsi, 7
add byte [rsi], 5
cmp byte [rsi], 0
LS_9:
jz LE_9
sub byte [rsi], 1
cmp byte [rsi], 0
LS_10:
jz LE_10
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_10
LE_10:
add rsi, 9
jmp LS_9
LE_9:
add rsi, 7
add byte [rsi], 1
add rsi, 26
add rsi, 1
add byte [rsi], 1
sub rsi, 17
cmp byte [rsi], 0
LS_11:
jz LE_11
sub rsi, 9
jmp LS_11
LE_11:
add rsi, 3
cmp byte [rsi], 0
LS_12:
jz LE_12
sub byte [rsi], 1
jmp LS_12
LE_12:
add byte [rsi], 1
cmp byte [rsi], 0
LS_13:
jz LE_13
add rsi, 6
cmp byte [rsi], 0
LS_14:
jz LE_14
add rsi, 7
cmp byte [rsi], 0
LS_15:
jz LE_15
sub byte [rsi], 1
jmp LS_15
LE_15:
add rsi, 2
jmp LS_14
LE_14:
sub rsi, 9
cmp byte [rsi], 0
LS_16:
jz LE_16
sub rsi, 9
jmp LS_16
LE_16:
add rsi, 2
add rsi, 5
cmp byte [rsi], 0
LS_17:
jz LE_17
sub byte [rsi], 1
jmp LS_17
LE_17:
add byte [rsi], 1
sub rsi, 6
add byte [rsi], 4
cmp byte [rsi], 0
LS_18:
jz LE_18
sub byte [rsi], 1
cmp byte [rsi], 0
LS_19:
jz LE_19
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_19
LE_19:
add rsi, 9
jmp LS_18
LE_18:
add rsi, 6
add byte [rsi], 1
sub rsi, 6
add byte [rsi], 7
cmp byte [rsi], 0
LS_20:
jz LE_20
sub byte [rsi], 1
cmp byte [rsi], 0
LS_21:
jz LE_21
sub byte [rsi], 1
add rsi, 3
add rsi, 6
add byte [rsi], 1
sub rsi, 9
jmp LS_21
LE_21:
add rsi, 9
jmp LS_20
LE_20:
add rsi, 6
add byte [rsi], 1
sub rsi, 16
cmp byte [rsi], 0
LS_22:
jz LE_22
sub rsi, 9
jmp LS_22
LE_22:
add rsi, 3
cmp byte [rsi], 0
LS_23:
jz LE_23
cmp byte [rsi], 0
LS_24:
jz LE_24
sub byte [rsi], 1
jmp LS_24
LE_24:
add rsi, 6
cmp byte [rsi], 0
LS_25:
jz LE_25
add rsi, 5
add rsi, 2
cmp byte [rsi], 0
LS_26:
jz LE_26
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_26
LE_26:
sub rsi, 6
cmp byte [rsi], 0
LS_27:
jz LE_27
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 1
jmp LS_27
LE_27:
add rsi, 8
jmp LS_25
LE_25:
sub rsi, 9
cmp byte [rsi], 0
LS_28:
jz LE_28
sub rsi, 9
jmp LS_28
LE_28:
add rsi, 9
cmp byte [rsi], 0
LS_29:
jz LE_29
add rsi, 8
cmp byte [rsi], 0
LS_30:
jz LE_30
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_30
LE_30:
sub rsi, 7
cmp byte [rsi], 0
LS_31:
jz LE_31
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 2
jmp LS_31
LE_31:
add rsi, 8
jmp LS_29
LE_29:
sub rsi, 9
cmp byte [rsi], 0
LS_32:
jz LE_32
sub rsi, 7
sub rsi, 2
jmp LS_32
LE_32:
add rsi, 7
cmp byte [rsi], 0
LS_33:
jz LE_33
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_33
LE_33:
sub rsi, 7
cmp byte [rsi], 0
LS_34:
jz LE_34
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
sub rsi, 5
jmp LS_34
LE_34:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_35:
jz LE_35
cmp byte [rsi], 0
LS_36:
jz LE_36
add rsi, 9
jmp LS_36
LE_36:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_37:
jz LE_37
sub byte [rsi], 1
jmp LS_37
LE_37:
add rsi, 1
cmp byte [rsi], 0
LS_38:
jz LE_38
sub byte [rsi], 1
jmp LS_38
LE_38:
add rsi, 1
cmp byte [rsi], 0
LS_39:
jz LE_39
sub byte [rsi], 1
jmp LS_39
LE_39:
add rsi, 1
cmp byte [rsi], 0
LS_40:
jz LE_40
sub byte [rsi], 1
jmp LS_40
LE_40:
add rsi, 1
cmp byte [rsi], 0
LS_41:
jz LE_41
sub byte [rsi], 1
jmp LS_41
LE_41:
add rsi, 1
cmp byte [rsi], 0
LS_42:
jz LE_42
sub byte [rsi], 1
jmp LS_42
LE_42:
add rsi, 1
cmp byte [rsi], 0
LS_43:
jz LE_43
sub byte [rsi], 1
jmp LS_43
LE_43:
add rsi, 1
cmp byte [rsi], 0
LS_44:
jz LE_44
sub byte [rsi], 1
jmp LS_44
LE_44:
add rsi, 1
cmp byte [rsi], 0
LS_45:
jz LE_45
sub byte [rsi], 1
jmp LS_45
LE_45:
sub rsi, 9
cmp byte [rsi], 0
LS_46:
jz LE_46
sub rsi, 9
jmp LS_46
LE_46:
add rsi, 9
sub byte [rsi], 1
jmp LS_35
LE_35:
add byte [rsi], 1
cmp byte [rsi], 0
LS_47:
jz LE_47
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_47
LE_47:
sub rsi, 9
cmp byte [rsi], 0
LS_48:
jz LE_48
sub rsi, 9
jmp LS_48
LE_48:
add rsi, 9
cmp byte [rsi], 0
LS_49:
jz LE_49
add rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_50:
jz LE_50
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_50
LE_50:
sub rsi, 4
cmp byte [rsi], 0
LS_51:
jz LE_51
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_52:
jz LE_52
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_53:
jz LE_53
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_53
LE_53:
sub rsi, 2
cmp byte [rsi], 0
LS_54:
jz LE_54
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 4
jmp LS_54
LE_54:
add byte [rsi], 1
add rsi, 9
jmp LS_52
LE_52:
sub rsi, 8
cmp byte [rsi], 0
LS_55:
jz LE_55
sub rsi, 9
jmp LS_55
LE_55:
jmp LS_51
LE_51:
add rsi, 9
cmp byte [rsi], 0
LS_56:
jz LE_56
add rsi, 9
jmp LS_56
LE_56:
sub rsi, 7
sub rsi, 2
cmp byte [rsi], 0
LS_57:
jz LE_57
add rsi, 1
cmp byte [rsi], 0
LS_58:
jz LE_58
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_58
LE_58:
sub rsi, 10
jmp LS_57
LE_57:
add rsi, 1
cmp byte [rsi], 0
LS_59:
jz LE_59
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_59
LE_59:
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_49
LE_49:
sub rsi, 9
cmp byte [rsi], 0
LS_60:
jz LE_60
add rsi, 1
cmp byte [rsi], 0
LS_61:
jz LE_61
sub byte [rsi], 1
jmp LS_61
LE_61:
sub rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_62:
jz LE_62
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_63:
jz LE_63
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_63
LE_63:
sub rsi, 1
cmp byte [rsi], 0
LS_64:
jz LE_64
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_64
LE_64:
add rsi, 4
jmp LS_62
LE_62:
sub rsi, 3
cmp byte [rsi], 0
LS_65:
jz LE_65
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_65
LE_65:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_60
LE_60:
add rsi, 5
add rsi, 4
cmp byte [rsi], 0
LS_66:
jz LE_66
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_66
LE_66:
sub rsi, 9
cmp byte [rsi], 0
LS_67:
jz LE_67
sub rsi, 9
jmp LS_67
LE_67:
add rsi, 9
cmp byte [rsi], 0
LS_68:
jz LE_68
add rsi, 1
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_69:
jz LE_69
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_69
LE_69:
sub rsi, 5
cmp byte [rsi], 0
LS_70:
jz LE_70
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_71:
jz LE_71
sub byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_72:
jz LE_72
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_72
LE_72:
sub rsi, 3
cmp byte [rsi], 0
LS_73:
jz LE_73
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 4
jmp LS_73
LE_73:
add byte [rsi], 1
add rsi, 9
jmp LS_71
LE_71:
sub rsi, 8
cmp byte [rsi], 0
LS_74:
jz LE_74
sub rsi, 9
jmp LS_74
LE_74:
jmp LS_70
LE_70:
add rsi, 9
cmp byte [rsi], 0
LS_75:
jz LE_75
add rsi, 2
add rsi, 7
jmp LS_75
LE_75:
sub rsi, 9
cmp byte [rsi], 0
LS_76:
jz LE_76
add rsi, 2
cmp byte [rsi], 0
LS_77:
jz LE_77
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_77
LE_77:
sub rsi, 11
jmp LS_76
LE_76:
add rsi, 2
cmp byte [rsi], 0
LS_78:
jz LE_78
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_78
LE_78:
sub rsi, 2
add byte [rsi], 1
add rsi, 8
jmp LS_68
LE_68:
sub rsi, 9
cmp byte [rsi], 0
LS_79:
jz LE_79
add rsi, 1
cmp byte [rsi], 0
LS_80:
jz LE_80
sub byte [rsi], 1
jmp LS_80
LE_80:
sub rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_81:
jz LE_81
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_82:
jz LE_82
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_82
LE_82:
sub rsi, 1
cmp byte [rsi], 0
LS_83:
jz LE_83
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_83
LE_83:
add rsi, 4
jmp LS_81
LE_81:
sub rsi, 3
cmp byte [rsi], 0
LS_84:
jz LE_84
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 2
sub rsi, 1
jmp LS_84
LE_84:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_79
LE_79:
add rsi, 9
cmp byte [rsi], 0
LS_85:
jz LE_85
add rsi, 4
cmp byte [rsi], 0
LS_86:
jz LE_86
sub byte [rsi], 1
sub rsi, 36
add byte [rsi], 1
add rsi, 13
add rsi, 23
jmp LS_86
LE_86:
add rsi, 5
jmp LS_85
LE_85:
sub rsi, 9
cmp byte [rsi], 0
LS_87:
jz LE_87
sub rsi, 9
jmp LS_87
LE_87:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_88:
jz LE_88
cmp byte [rsi], 0
LS_89:
jz LE_89
add rsi, 4
add rsi, 5
jmp LS_89
LE_89:
sub rsi, 9
sub byte [rsi], 1
sub rsi, 9
cmp byte [rsi], 0
LS_90:
jz LE_90
sub rsi, 9
jmp LS_90
LE_90:
add rsi, 9
sub byte [rsi], 1
jmp LS_88
LE_88:
add byte [rsi], 1
add rsi, 21
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_91:
jz LE_91
sub rsi, 6
sub rsi, 3
jmp LS_91
LE_91:
add rsi, 9
cmp byte [rsi], 0
LS_92:
jz LE_92
add rsi, 3
cmp byte [rsi], 0
LS_93:
jz LE_93
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_93
LE_93:
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_94:
jz LE_94
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_95:
jz LE_95
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_95
LE_95:
sub rsi, 4
cmp byte [rsi], 0
LS_96:
jz LE_96
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 13
cmp byte [rsi], 0
LS_97:
jz LE_97
sub rsi, 5
sub rsi, 4
jmp LS_97
LE_97:
add rsi, 4
cmp byte [rsi], 0
LS_98:
jz LE_98
sub byte [rsi], 1
jmp LS_98
LE_98:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_99:
jz LE_99
add rsi, 9
jmp LS_99
LE_99:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_96
LE_96:
jmp LS_94
LE_94:
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_100:
jz LE_100
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_100
LE_100:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_101:
jz LE_101
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_102:
jz LE_102
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_102
LE_102:
sub rsi, 3
cmp byte [rsi], 0
LS_103:
jz LE_103
sub byte [rsi], 1
add rsi, 1
add rsi, 2
add byte [rsi], 1
sub rsi, 12
cmp byte [rsi], 0
LS_104:
jz LE_104
sub rsi, 9
jmp LS_104
LE_104:
add rsi, 3
cmp byte [rsi], 0
LS_105:
jz LE_105
sub byte [rsi], 1
jmp LS_105
LE_105:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_106:
jz LE_106
add rsi, 9
jmp LS_106
LE_106:
add rsi, 1
cmp byte [rsi], 0
LS_107:
jz LE_107
sub byte [rsi], 1
jmp LS_107
LE_107:
add byte [rsi], 1
sub rsi, 1
jmp LS_103
LE_103:
jmp LS_101
LE_101:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_108:
jz LE_108
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_109:
jz LE_109
add rsi, 9
jmp LS_109
LE_109:
sub rsi, 6
sub rsi, 2
jmp LS_108
LE_108:
add rsi, 8
jmp LS_92
LE_92:
sub rsi, 9
cmp byte [rsi], 0
LS_110:
jz LE_110
sub rsi, 9
jmp LS_110
LE_110:
sub rsi, 7
cmp byte [rsi], 0
LS_111:
jz LE_111
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 4
jmp LS_111
LE_111:
add rsi, 9
add byte [rsi], 19
add byte [rsi], 7
add rsi, 2
cmp byte [rsi], 0
LS_112:
jz LE_112
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_112
LE_112:
sub rsi, 4
cmp byte [rsi], 0
LS_113:
jz LE_113
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_114:
jz LE_114
sub byte [rsi], 1
jmp LS_114
LE_114:
sub rsi, 2
jmp LS_113
LE_113:
add rsi, 2
cmp byte [rsi], 0
LS_115:
jz LE_115
sub rsi, 7
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_116:
jz LE_116
sub byte [rsi], 1
sub rsi, 1
add byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_117:
jz LE_117
sub byte [rsi], 1
jmp LS_117
LE_117:
jmp LS_116
LE_116:
add rsi, 1
cmp byte [rsi], 0
LS_118:
jz LE_118
sub byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_119:
jz LE_119
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 4
jmp LS_119
LE_119:
add rsi, 3
jmp LS_118
LE_118:
add rsi, 13
cmp byte [rsi], 0
LS_120:
jz LE_120
add rsi, 2
cmp byte [rsi], 0
LS_121:
jz LE_121
sub byte [rsi], 1
jmp LS_121
LE_121:
add rsi, 1
cmp byte [rsi], 0
LS_122:
jz LE_122
sub byte [rsi], 1
jmp LS_122
LE_122:
add rsi, 1
cmp byte [rsi], 0
LS_123:
jz LE_123
sub byte [rsi], 1
jmp LS_123
LE_123:
add rsi, 5
jmp LS_120
LE_120:
sub rsi, 9
cmp byte [rsi], 0
LS_124:
jz LE_124
sub rsi, 9
jmp LS_124
LE_124:
add rsi, 3
cmp byte [rsi], 0
LS_125:
jz LE_125
sub byte [rsi], 1
jmp LS_125
LE_125:
add rsi, 6
cmp byte [rsi], 0
LS_126:
jz LE_126
add rsi, 5
cmp byte [rsi], 0
LS_127:
jz LE_127
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_127
LE_127:
sub rsi, 4
cmp byte [rsi], 0
LS_128:
jz LE_128
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 1
jmp LS_128
LE_128:
add rsi, 8
jmp LS_126
LE_126:
sub rsi, 9
cmp byte [rsi], 0
LS_129:
jz LE_129
sub rsi, 9
jmp LS_129
LE_129:
add rsi, 9
cmp byte [rsi], 0
LS_130:
jz LE_130
add rsi, 2
cmp byte [rsi], 0
LS_131:
jz LE_131
sub byte [rsi], 1
sub rsi, 8
sub rsi, 1
add byte [rsi], 1
add rsi, 9
jmp LS_131
LE_131:
add rsi, 7
jmp LS_130
LE_130:
sub rsi, 9
cmp byte [rsi], 0
LS_132:
jz LE_132
sub rsi, 9
jmp LS_132
LE_132:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_133:
jz LE_133
cmp byte [rsi], 0
LS_134:
jz LE_134
add rsi, 9
jmp LS_134
LE_134:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_135:
jz LE_135
sub byte [rsi], 1
jmp LS_135
LE_135:
add rsi, 1
cmp byte [rsi], 0
LS_136:
jz LE_136
sub byte [rsi], 1
jmp LS_136
LE_136:
add rsi, 1
cmp byte [rsi], 0
LS_137:
jz LE_137
sub byte [rsi], 1
jmp LS_137
LE_137:
add rsi, 1
cmp byte [rsi], 0
LS_138:
jz LE_138
sub byte [rsi], 1
jmp LS_138
LE_138:
add rsi, 1
cmp byte [rsi], 0
LS_139:
jz LE_139
sub byte [rsi], 1
jmp LS_139
LE_139:
add rsi, 1
cmp byte [rsi], 0
LS_140:
jz LE_140
sub byte [rsi], 1
jmp LS_140
LE_140:
add rsi, 1
cmp byte [rsi], 0
LS_141:
jz LE_141
sub byte [rsi], 1
jmp LS_141
LE_141:
add rsi, 1
cmp byte [rsi], 0
LS_142:
jz LE_142
sub byte [rsi], 1
jmp LS_142
LE_142:
add rsi, 1
cmp byte [rsi], 0
LS_143:
jz LE_143
sub byte [rsi], 1
jmp LS_143
LE_143:
sub rsi, 9
cmp byte [rsi], 0
LS_144:
jz LE_144
sub rsi, 9
jmp LS_144
LE_144:
add rsi, 9
sub byte [rsi], 1
jmp LS_133
LE_133:
add byte [rsi], 1
cmp byte [rsi], 0
LS_145:
jz LE_145
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_145
LE_145:
sub rsi, 3
sub rsi, 6
cmp byte [rsi], 0
LS_146:
jz LE_146
sub rsi, 9
jmp LS_146
LE_146:
add rsi, 9
cmp byte [rsi], 0
LS_147:
jz LE_147
add rsi, 1
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_148:
jz LE_148
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_148
LE_148:
sub rsi, 5
cmp byte [rsi], 0
LS_149:
jz LE_149
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_150:
jz LE_150
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_151:
jz LE_151
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_151
LE_151:
sub rsi, 1
sub rsi, 1
cmp byte [rsi], 0
LS_152:
jz LE_152
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 3
jmp LS_152
LE_152:
add byte [rsi], 1
add rsi, 9
jmp LS_150
LE_150:
sub rsi, 8
cmp byte [rsi], 0
LS_153:
jz LE_153
sub rsi, 9
jmp LS_153
LE_153:
jmp LS_149
LE_149:
add rsi, 9
cmp byte [rsi], 0
LS_154:
jz LE_154
add rsi, 9
jmp LS_154
LE_154:
sub rsi, 9
cmp byte [rsi], 0
LS_155:
jz LE_155
add rsi, 1
cmp byte [rsi], 0
LS_156:
jz LE_156
sub byte [rsi], 1
add rsi, 4
add rsi, 5
add byte [rsi], 1
sub rsi, 9
jmp LS_156
LE_156:
sub rsi, 10
jmp LS_155
LE_155:
add rsi, 1
cmp byte [rsi], 0
LS_157:
jz LE_157
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_157
LE_157:
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_147
LE_147:
sub rsi, 9
cmp byte [rsi], 0
LS_158:
jz LE_158
add rsi, 1
cmp byte [rsi], 0
LS_159:
jz LE_159
sub byte [rsi], 1
jmp LS_159
LE_159:
sub rsi, 1
sub byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_160:
jz LE_160
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_161:
jz LE_161
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_161
LE_161:
sub rsi, 1
cmp byte [rsi], 0
LS_162:
jz LE_162
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_162
LE_162:
add rsi, 3
jmp LS_160
LE_160:
sub rsi, 2
cmp byte [rsi], 0
LS_163:
jz LE_163
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_163
LE_163:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_158
LE_158:
add rsi, 9
cmp byte [rsi], 0
LS_164:
jz LE_164
add rsi, 6
cmp byte [rsi], 0
LS_165:
jz LE_165
sub byte [rsi], 1
sub rsi, 1
sub rsi, 4
add byte [rsi], 1
add rsi, 5
jmp LS_165
LE_165:
sub rsi, 5
cmp byte [rsi], 0
LS_166:
jz LE_166
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
sub rsi, 1
jmp LS_166
LE_166:
add rsi, 8
jmp LS_164
LE_164:
sub rsi, 9
cmp byte [rsi], 0
LS_167:
jz LE_167
sub rsi, 9
jmp LS_167
LE_167:
add rsi, 9
cmp byte [rsi], 0
LS_168:
jz LE_168
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_168
LE_168:
sub rsi, 9
cmp byte [rsi], 0
LS_169:
jz LE_169
sub rsi, 9
jmp LS_169
LE_169:
add rsi, 9
cmp byte [rsi], 0
LS_170:
jz LE_170
add rsi, 1
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_171:
jz LE_171
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_171
LE_171:
sub rsi, 5
cmp byte [rsi], 0
LS_172:
jz LE_172
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_173:
jz LE_173
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_174:
jz LE_174
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_174
LE_174:
sub rsi, 2
cmp byte [rsi], 0
LS_175:
jz LE_175
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 4
jmp LS_175
LE_175:
add byte [rsi], 1
add rsi, 9
jmp LS_173
LE_173:
sub rsi, 8
cmp byte [rsi], 0
LS_176:
jz LE_176
sub rsi, 9
jmp LS_176
LE_176:
jmp LS_172
LE_172:
add rsi, 9
cmp byte [rsi], 0
LS_177:
jz LE_177
add rsi, 9
jmp LS_177
LE_177:
sub rsi, 9
cmp byte [rsi], 0
LS_178:
jz LE_178
add rsi, 1
cmp byte [rsi], 0
LS_179:
jz LE_179
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_179
LE_179:
sub rsi, 10
jmp LS_178
LE_178:
add rsi, 1
cmp byte [rsi], 0
LS_180:
jz LE_180
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_180
LE_180:
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_170
LE_170:
sub rsi, 9
cmp byte [rsi], 0
LS_181:
jz LE_181
add rsi, 1
cmp byte [rsi], 0
LS_182:
jz LE_182
sub byte [rsi], 1
jmp LS_182
LE_182:
sub rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_183:
jz LE_183
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_184:
jz LE_184
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_184
LE_184:
sub rsi, 1
cmp byte [rsi], 0
LS_185:
jz LE_185
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_185
LE_185:
add rsi, 4
jmp LS_183
LE_183:
sub rsi, 3
cmp byte [rsi], 0
LS_186:
jz LE_186
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_186
LE_186:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_181
LE_181:
add rsi, 9
cmp byte [rsi], 0
LS_187:
jz LE_187
add rsi, 4
cmp byte [rsi], 0
LS_188:
jz LE_188
sub byte [rsi], 1
sub rsi, 36
add byte [rsi], 1
add rsi, 36
jmp LS_188
LE_188:
add rsi, 5
jmp LS_187
LE_187:
sub rsi, 9
cmp byte [rsi], 0
LS_189:
jz LE_189
sub rsi, 9
jmp LS_189
LE_189:
add rsi, 9
cmp byte [rsi], 0
LS_190:
jz LE_190
add rsi, 3
cmp byte [rsi], 0
LS_191:
jz LE_191
sub byte [rsi], 1
sub rsi, 36
add byte [rsi], 1
add rsi, 1
add rsi, 35
jmp LS_191
LE_191:
add rsi, 6
jmp LS_190
LE_190:
sub rsi, 9
cmp byte [rsi], 0
LS_192:
jz LE_192
sub rsi, 9
jmp LS_192
LE_192:
add rsi, 9
add byte [rsi], 8
add byte [rsi], 7
cmp byte [rsi], 0
LS_193:
jz LE_193
cmp byte [rsi], 0
LS_194:
jz LE_194
add rsi, 9
jmp LS_194
LE_194:
sub rsi, 9
sub byte [rsi], 1
sub rsi, 9
cmp byte [rsi], 0
LS_195:
jz LE_195
sub rsi, 9
jmp LS_195
LE_195:
add rsi, 9
sub byte [rsi], 1
jmp LS_193
LE_193:
add byte [rsi], 1
cmp byte [rsi], 0
LS_196:
jz LE_196
add rsi, 8
cmp byte [rsi], 0
LS_197:
jz LE_197
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_197
LE_197:
sub rsi, 7
cmp byte [rsi], 0
LS_198:
jz LE_198
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
sub rsi, 1
jmp LS_198
LE_198:
add rsi, 8
jmp LS_196
LE_196:
sub rsi, 9
cmp byte [rsi], 0
LS_199:
jz LE_199
sub rsi, 9
jmp LS_199
LE_199:
add rsi, 9
cmp byte [rsi], 0
LS_200:
jz LE_200
add rsi, 6
cmp byte [rsi], 0
LS_201:
jz LE_201
sub byte [rsi], 1
jmp LS_201
LE_201:
add rsi, 3
jmp LS_200
LE_200:
sub rsi, 9
cmp byte [rsi], 0
LS_202:
jz LE_202
sub rsi, 9
jmp LS_202
LE_202:
add rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_203:
jz LE_203
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 5
jmp LS_203
LE_203:
add rsi, 1
cmp byte [rsi], 0
LS_204:
jz LE_204
sub byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_205:
jz LE_205
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 1
add byte [rsi], 2
sub rsi, 4
jmp LS_205
LE_205:
add rsi, 5
cmp byte [rsi], 0
LS_206:
jz LE_206
sub byte [rsi], 1
sub rsi, 1
sub rsi, 4
add byte [rsi], 1
add rsi, 5
jmp LS_206
LE_206:
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 1
jmp LS_204
LE_204:
sub rsi, 1
cmp byte [rsi], 0
LS_207:
jz LE_207
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_207
LE_207:
sub rsi, 5
cmp byte [rsi], 0
LS_208:
jz LE_208
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 5
jmp LS_208
LE_208:
add rsi, 6
cmp byte [rsi], 0
LS_209:
jz LE_209
sub byte [rsi], 1
jmp LS_209
LE_209:
sub rsi, 6
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_210:
jz LE_210
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_210
LE_210:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_211:
jz LE_211
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_212:
jz LE_212
add rsi, 2
cmp byte [rsi], 0
LS_213:
jz LE_213
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
add rsi, 2
jmp LS_213
LE_213:
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_214:
jz LE_214
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_215:
jz LE_215
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_215
LE_215:
sub rsi, 3
cmp byte [rsi], 0
LS_216:
jz LE_216
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 12
cmp byte [rsi], 0
LS_217:
jz LE_217
sub rsi, 9
jmp LS_217
LE_217:
add rsi, 3
cmp byte [rsi], 0
LS_218:
jz LE_218
sub byte [rsi], 1
jmp LS_218
LE_218:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_219:
jz LE_219
add rsi, 9
jmp LS_219
LE_219:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_216
LE_216:
jmp LS_214
LE_214:
add byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_220:
jz LE_220
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_220
LE_220:
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_221:
jz LE_221
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_222:
jz LE_222
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_222
LE_222:
sub rsi, 2
cmp byte [rsi], 0
LS_223:
jz LE_223
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 11
cmp byte [rsi], 0
LS_224:
jz LE_224
sub rsi, 5
sub rsi, 4
jmp LS_224
LE_224:
add rsi, 4
cmp byte [rsi], 0
LS_225:
jz LE_225
sub byte [rsi], 1
jmp LS_225
LE_225:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_226:
jz LE_226
add rsi, 9
jmp LS_226
LE_226:
add rsi, 1
cmp byte [rsi], 0
LS_227:
jz LE_227
sub byte [rsi], 1
jmp LS_227
LE_227:
add byte [rsi], 1
sub rsi, 1
jmp LS_223
LE_223:
jmp LS_221
LE_221:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_228:
jz LE_228
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_229:
jz LE_229
add rsi, 9
jmp LS_229
LE_229:
sub rsi, 8
jmp LS_228
LE_228:
add rsi, 8
jmp LS_212
LE_212:
sub rsi, 9
cmp byte [rsi], 0
LS_230:
jz LE_230
sub rsi, 9
jmp LS_230
LE_230:
add rsi, 4
cmp byte [rsi], 0
LS_231:
jz LE_231
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_231
LE_231:
sub rsi, 4
cmp byte [rsi], 0
LS_232:
jz LE_232
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_233:
jz LE_233
add rsi, 1
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_234:
jz LE_234
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
add rsi, 2
jmp LS_234
LE_234:
sub rsi, 2
cmp byte [rsi], 0
LS_235:
jz LE_235
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_235
LE_235:
add rsi, 8
jmp LS_233
LE_233:
sub rsi, 5
sub rsi, 3
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_236:
jz LE_236
add rsi, 1
cmp byte [rsi], 0
LS_237:
jz LE_237
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_238:
jz LE_238
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 11
cmp byte [rsi], 0
LS_239:
jz LE_239
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_239
LE_239:
sub rsi, 1
jmp LS_238
LE_238:
add rsi, 1
cmp byte [rsi], 0
LS_240:
jz LE_240
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 9
sub rsi, 5
add byte [rsi], 1
add rsi, 11
jmp LS_240
LE_240:
sub rsi, 2
jmp LS_237
LE_237:
add rsi, 1
cmp byte [rsi], 0
LS_241:
jz LE_241
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_242:
jz LE_242
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 11
jmp LS_242
LE_242:
sub rsi, 1
jmp LS_241
LE_241:
add rsi, 1
cmp byte [rsi], 0
LS_243:
jz LE_243
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_243
LE_243:
sub rsi, 2
sub rsi, 10
jmp LS_236
LE_236:
add rsi, 4
cmp byte [rsi], 0
LS_244:
jz LE_244
sub byte [rsi], 1
jmp LS_244
LE_244:
sub rsi, 4
jmp LS_232
LE_232:
add rsi, 3
cmp byte [rsi], 0
LS_245:
jz LE_245
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_245
LE_245:
sub rsi, 3
cmp byte [rsi], 0
LS_246:
jz LE_246
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_247:
jz LE_247
add rsi, 1
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_248:
jz LE_248
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
jmp LS_248
LE_248:
sub rsi, 1
cmp byte [rsi], 0
LS_249:
jz LE_249
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_249
LE_249:
add rsi, 8
jmp LS_247
LE_247:
sub rsi, 3
sub rsi, 5
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_250:
jz LE_250
add rsi, 1
cmp byte [rsi], 0
LS_251:
jz LE_251
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_252:
jz LE_252
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 10
cmp byte [rsi], 0
LS_253:
jz LE_253
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 4
jmp LS_253
LE_253:
add rsi, 1
jmp LS_252
LE_252:
sub rsi, 1
cmp byte [rsi], 0
LS_254:
jz LE_254
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 7
sub rsi, 7
add byte [rsi], 1
add rsi, 10
jmp LS_254
LE_254:
sub rsi, 1
jmp LS_251
LE_251:
add rsi, 2
cmp byte [rsi], 0
LS_255:
jz LE_255
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_256:
jz LE_256
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 10
jmp LS_256
LE_256:
add rsi, 1
jmp LS_255
LE_255:
sub rsi, 1
cmp byte [rsi], 0
LS_257:
jz LE_257
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 4
jmp LS_257
LE_257:
sub rsi, 11
jmp LS_250
LE_250:
add rsi, 6
add byte [rsi], 1
sub rsi, 6
jmp LS_246
LE_246:
jmp LS_211
LE_211:
add rsi, 4
cmp byte [rsi], 0
LS_258:
jz LE_258
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_258
LE_258:
sub rsi, 4
cmp byte [rsi], 0
LS_259:
jz LE_259
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_260:
jz LE_260
add rsi, 9
jmp LS_260
LE_260:
sub rsi, 9
cmp byte [rsi], 0
LS_261:
jz LE_261
add rsi, 1
cmp byte [rsi], 0
LS_262:
jz LE_262
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_263:
jz LE_263
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 11
cmp byte [rsi], 0
LS_264:
jz LE_264
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_264
LE_264:
sub rsi, 1
jmp LS_263
LE_263:
add rsi, 1
cmp byte [rsi], 0
LS_265:
jz LE_265
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 11
jmp LS_265
LE_265:
sub rsi, 2
jmp LS_262
LE_262:
add rsi, 1
cmp byte [rsi], 0
LS_266:
jz LE_266
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_267:
jz LE_267
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 14
add byte [rsi], 1
add rsi, 11
jmp LS_267
LE_267:
sub rsi, 1
jmp LS_266
LE_266:
add rsi, 1
cmp byte [rsi], 0
LS_268:
jz LE_268
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_268
LE_268:
sub rsi, 7
sub rsi, 5
jmp LS_261
LE_261:
jmp LS_259
LE_259:
add rsi, 1
cmp byte [rsi], 0
LS_269:
jz LE_269
sub byte [rsi], 1
jmp LS_269
LE_269:
add rsi, 2
cmp byte [rsi], 0
LS_270:
jz LE_270
sub byte [rsi], 1
jmp LS_270
LE_270:
add rsi, 1
cmp byte [rsi], 0
LS_271:
jz LE_271
sub byte [rsi], 1
jmp LS_271
LE_271:
add rsi, 5
cmp byte [rsi], 0
LS_272:
jz LE_272
add rsi, 2
cmp byte [rsi], 0
LS_273:
jz LE_273
sub byte [rsi], 1
jmp LS_273
LE_273:
add rsi, 1
cmp byte [rsi], 0
LS_274:
jz LE_274
sub byte [rsi], 1
jmp LS_274
LE_274:
add rsi, 6
jmp LS_272
LE_272:
sub rsi, 9
cmp byte [rsi], 0
LS_275:
jz LE_275
sub rsi, 9
jmp LS_275
LE_275:
add rsi, 9
cmp byte [rsi], 0
LS_276:
jz LE_276
add rsi, 5
cmp byte [rsi], 0
LS_277:
jz LE_277
sub byte [rsi], 1
sub rsi, 1
sub rsi, 3
add byte [rsi], 1
add rsi, 4
jmp LS_277
LE_277:
sub rsi, 4
cmp byte [rsi], 0
LS_278:
jz LE_278
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 1
jmp LS_278
LE_278:
add rsi, 8
jmp LS_276
LE_276:
sub rsi, 9
cmp byte [rsi], 0
LS_279:
jz LE_279
sub rsi, 9
jmp LS_279
LE_279:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_280:
jz LE_280
cmp byte [rsi], 0
LS_281:
jz LE_281
add rsi, 9
jmp LS_281
LE_281:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_282:
jz LE_282
sub byte [rsi], 1
jmp LS_282
LE_282:
add rsi, 1
cmp byte [rsi], 0
LS_283:
jz LE_283
sub byte [rsi], 1
jmp LS_283
LE_283:
add rsi, 1
cmp byte [rsi], 0
LS_284:
jz LE_284
sub byte [rsi], 1
jmp LS_284
LE_284:
add rsi, 1
cmp byte [rsi], 0
LS_285:
jz LE_285
sub byte [rsi], 1
jmp LS_285
LE_285:
add rsi, 1
cmp byte [rsi], 0
LS_286:
jz LE_286
sub byte [rsi], 1
jmp LS_286
LE_286:
add rsi, 1
cmp byte [rsi], 0
LS_287:
jz LE_287
sub byte [rsi], 1
jmp LS_287
LE_287:
add rsi, 1
cmp byte [rsi], 0
LS_288:
jz LE_288
sub byte [rsi], 1
jmp LS_288
LE_288:
add rsi, 1
cmp byte [rsi], 0
LS_289:
jz LE_289
sub byte [rsi], 1
jmp LS_289
LE_289:
add rsi, 1
cmp byte [rsi], 0
LS_290:
jz LE_290
sub byte [rsi], 1
jmp LS_290
LE_290:
sub rsi, 9
cmp byte [rsi], 0
LS_291:
jz LE_291
sub rsi, 9
jmp LS_291
LE_291:
add rsi, 9
sub byte [rsi], 1
jmp LS_280
LE_280:
add byte [rsi], 1
cmp byte [rsi], 0
LS_292:
jz LE_292
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_292
LE_292:
sub rsi, 9
cmp byte [rsi], 0
LS_293:
jz LE_293
sub rsi, 9
jmp LS_293
LE_293:
add rsi, 9
cmp byte [rsi], 0
LS_294:
jz LE_294
add rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_295:
jz LE_295
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_295
LE_295:
sub rsi, 4
cmp byte [rsi], 0
LS_296:
jz LE_296
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_297:
jz LE_297
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_298:
jz LE_298
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_298
LE_298:
sub rsi, 2
cmp byte [rsi], 0
LS_299:
jz LE_299
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 3
jmp LS_299
LE_299:
add byte [rsi], 1
add rsi, 9
jmp LS_297
LE_297:
sub rsi, 8
cmp byte [rsi], 0
LS_300:
jz LE_300
sub rsi, 9
jmp LS_300
LE_300:
jmp LS_296
LE_296:
add rsi, 9
cmp byte [rsi], 0
LS_301:
jz LE_301
add rsi, 9
jmp LS_301
LE_301:
sub rsi, 8
sub rsi, 1
cmp byte [rsi], 0
LS_302:
jz LE_302
add rsi, 1
cmp byte [rsi], 0
LS_303:
jz LE_303
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_303
LE_303:
sub rsi, 10
jmp LS_302
LE_302:
add rsi, 1
cmp byte [rsi], 0
LS_304:
jz LE_304
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_304
LE_304:
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_294
LE_294:
sub rsi, 9
cmp byte [rsi], 0
LS_305:
jz LE_305
add rsi, 1
cmp byte [rsi], 0
LS_306:
jz LE_306
sub byte [rsi], 1
jmp LS_306
LE_306:
sub rsi, 1
sub byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_307:
jz LE_307
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_308:
jz LE_308
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_308
LE_308:
sub rsi, 1
cmp byte [rsi], 0
LS_309:
jz LE_309
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_309
LE_309:
add rsi, 3
jmp LS_307
LE_307:
sub rsi, 2
cmp byte [rsi], 0
LS_310:
jz LE_310
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_310
LE_310:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_305
LE_305:
add rsi, 9
cmp byte [rsi], 0
LS_311:
jz LE_311
add rsi, 3
cmp byte [rsi], 0
LS_312:
jz LE_312
sub byte [rsi], 1
sub rsi, 36
add byte [rsi], 1
add rsi, 36
jmp LS_312
LE_312:
add rsi, 1
add rsi, 5
jmp LS_311
LE_311:
sub rsi, 9
cmp byte [rsi], 0
LS_313:
jz LE_313
sub rsi, 9
jmp LS_313
LE_313:
add rsi, 5
cmp byte [rsi], 0
LS_314:
jz LE_314
sub byte [rsi], 1
jmp LS_314
LE_314:
add rsi, 4
add byte [rsi], 15
cmp byte [rsi], 0
LS_315:
jz LE_315
cmp byte [rsi], 0
LS_316:
jz LE_316
add rsi, 9
jmp LS_316
LE_316:
sub rsi, 9
sub byte [rsi], 1
sub rsi, 5
sub rsi, 4
cmp byte [rsi], 0
LS_317:
jz LE_317
sub rsi, 9
jmp LS_317
LE_317:
add rsi, 9
sub byte [rsi], 1
jmp LS_315
LE_315:
add byte [rsi], 1
cmp byte [rsi], 0
LS_318:
jz LE_318
add rsi, 3
cmp byte [rsi], 0
LS_319:
jz LE_319
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_319
LE_319:
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_320:
jz LE_320
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_321:
jz LE_321
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_321
LE_321:
sub rsi, 4
cmp byte [rsi], 0
LS_322:
jz LE_322
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 5
sub rsi, 8
cmp byte [rsi], 0
LS_323:
jz LE_323
sub rsi, 9
jmp LS_323
LE_323:
add rsi, 4
cmp byte [rsi], 0
LS_324:
jz LE_324
sub byte [rsi], 1
jmp LS_324
LE_324:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_325:
jz LE_325
add rsi, 9
jmp LS_325
LE_325:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_322
LE_322:
jmp LS_320
LE_320:
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_326:
jz LE_326
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_326
LE_326:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_327:
jz LE_327
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_328:
jz LE_328
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_328
LE_328:
sub rsi, 3
cmp byte [rsi], 0
LS_329:
jz LE_329
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 12
cmp byte [rsi], 0
LS_330:
jz LE_330
sub rsi, 9
jmp LS_330
LE_330:
add rsi, 3
cmp byte [rsi], 0
LS_331:
jz LE_331
sub byte [rsi], 1
jmp LS_331
LE_331:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_332:
jz LE_332
add rsi, 9
jmp LS_332
LE_332:
add rsi, 1
cmp byte [rsi], 0
LS_333:
jz LE_333
sub byte [rsi], 1
jmp LS_333
LE_333:
add byte [rsi], 1
sub rsi, 1
jmp LS_329
LE_329:
jmp LS_327
LE_327:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_334:
jz LE_334
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_335:
jz LE_335
add rsi, 2
add rsi, 7
jmp LS_335
LE_335:
sub rsi, 8
jmp LS_334
LE_334:
add rsi, 8
jmp LS_318
LE_318:
sub rsi, 9
cmp byte [rsi], 0
LS_336:
jz LE_336
sub rsi, 9
jmp LS_336
LE_336:
add rsi, 3
cmp byte [rsi], 0
LS_337:
jz LE_337
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_337
LE_337:
sub rsi, 3
cmp byte [rsi], 0
LS_338:
jz LE_338
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_339:
jz LE_339
add rsi, 1
add byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_340:
jz LE_340
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_340
LE_340:
sub rsi, 3
cmp byte [rsi], 0
LS_341:
jz LE_341
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_341
LE_341:
add rsi, 8
jmp LS_339
LE_339:
sub rsi, 8
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_342:
jz LE_342
add rsi, 1
cmp byte [rsi], 0
LS_343:
jz LE_343
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_344:
jz LE_344
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 12
cmp byte [rsi], 0
LS_345:
jz LE_345
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_345
LE_345:
sub rsi, 1
jmp LS_344
LE_344:
add rsi, 1
cmp byte [rsi], 0
LS_346:
jz LE_346
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 12
jmp LS_346
LE_346:
sub rsi, 3
jmp LS_343
LE_343:
add rsi, 2
cmp byte [rsi], 0
LS_347:
jz LE_347
sub byte [rsi], 1
sub rsi, 1
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_348:
jz LE_348
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 12
jmp LS_348
LE_348:
sub rsi, 1
jmp LS_347
LE_347:
add rsi, 1
cmp byte [rsi], 0
LS_349:
jz LE_349
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_349
LE_349:
sub rsi, 13
jmp LS_342
LE_342:
jmp LS_338
LE_338:
add rsi, 4
cmp byte [rsi], 0
LS_350:
jz LE_350
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_350
LE_350:
sub rsi, 4
cmp byte [rsi], 0
LS_351:
jz LE_351
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_352:
jz LE_352
add rsi, 1
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_353:
jz LE_353
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
add rsi, 2
jmp LS_353
LE_353:
sub rsi, 2
cmp byte [rsi], 0
LS_354:
jz LE_354
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_354
LE_354:
add rsi, 2
add rsi, 6
jmp LS_352
LE_352:
sub rsi, 8
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_355:
jz LE_355
add rsi, 1
cmp byte [rsi], 0
LS_356:
jz LE_356
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_357:
jz LE_357
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 11
cmp byte [rsi], 0
LS_358:
jz LE_358
sub byte [rsi], 1
sub rsi, 1
add byte [rsi], 1
add rsi, 1
jmp LS_358
LE_358:
add rsi, 1
jmp LS_357
LE_357:
sub rsi, 1
cmp byte [rsi], 0
LS_359:
jz LE_359
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 4
add rsi, 7
jmp LS_359
LE_359:
sub rsi, 2
jmp LS_356
LE_356:
add rsi, 3
cmp byte [rsi], 0
LS_360:
jz LE_360
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_361:
jz LE_361
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
sub rsi, 10
add byte [rsi], 1
add rsi, 11
jmp LS_361
LE_361:
add rsi, 1
jmp LS_360
LE_360:
sub rsi, 1
cmp byte [rsi], 0
LS_362:
jz LE_362
sub byte [rsi], 1
sub rsi, 1
add byte [rsi], 1
add rsi, 1
jmp LS_362
LE_362:
sub rsi, 12
jmp LS_355
LE_355:
add rsi, 5
add byte [rsi], 1
sub rsi, 5
jmp LS_351
LE_351:
add rsi, 9
cmp byte [rsi], 0
LS_363:
jz LE_363
add rsi, 3
cmp byte [rsi], 0
LS_364:
jz LE_364
sub byte [rsi], 1
jmp LS_364
LE_364:
add rsi, 1
cmp byte [rsi], 0
LS_365:
jz LE_365
sub byte [rsi], 1
jmp LS_365
LE_365:
add rsi, 1
cmp byte [rsi], 0
LS_366:
jz LE_366
sub byte [rsi], 1
jmp LS_366
LE_366:
add rsi, 4
jmp LS_363
LE_363:
sub rsi, 9
cmp byte [rsi], 0
LS_367:
jz LE_367
sub rsi, 9
jmp LS_367
LE_367:
add rsi, 3
cmp byte [rsi], 0
LS_368:
jz LE_368
sub byte [rsi], 1
jmp LS_368
LE_368:
add rsi, 1
cmp byte [rsi], 0
LS_369:
jz LE_369
sub byte [rsi], 1
jmp LS_369
LE_369:
add rsi, 5
cmp byte [rsi], 0
LS_370:
jz LE_370
add rsi, 7
cmp byte [rsi], 0
LS_371:
jz LE_371
sub byte [rsi], 1
sub rsi, 5
sub rsi, 1
add byte [rsi], 1
add rsi, 6
jmp LS_371
LE_371:
sub rsi, 6
cmp byte [rsi], 0
LS_372:
jz LE_372
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
sub rsi, 2
jmp LS_372
LE_372:
add rsi, 8
jmp LS_370
LE_370:
sub rsi, 9
cmp byte [rsi], 0
LS_373:
jz LE_373
sub rsi, 9
jmp LS_373
LE_373:
add rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_374:
jz LE_374
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
add rsi, 1
jmp LS_374
LE_374:
add rsi, 2
cmp byte [rsi], 0
LS_375:
jz LE_375
sub byte [rsi], 1
sub rsi, 7
cmp byte [rsi], 0
LS_376:
jz LE_376
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 1
add byte [rsi], 2
sub rsi, 4
jmp LS_376
LE_376:
add rsi, 5
cmp byte [rsi], 0
LS_377:
jz LE_377
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_377
LE_377:
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
add rsi, 2
jmp LS_375
LE_375:
sub rsi, 2
cmp byte [rsi], 0
LS_378:
jz LE_378
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_378
LE_378:
sub rsi, 5
cmp byte [rsi], 0
LS_379:
jz LE_379
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 2
sub rsi, 3
jmp LS_379
LE_379:
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_380:
jz LE_380
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_380
LE_380:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_381:
jz LE_381
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_382:
jz LE_382
add rsi, 3
cmp byte [rsi], 0
LS_383:
jz LE_383
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_383
LE_383:
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_384:
jz LE_384
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_385:
jz LE_385
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_385
LE_385:
sub rsi, 2
cmp byte [rsi], 0
LS_386:
jz LE_386
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
sub rsi, 9
cmp byte [rsi], 0
LS_387:
jz LE_387
sub rsi, 9
jmp LS_387
LE_387:
add rsi, 4
cmp byte [rsi], 0
LS_388:
jz LE_388
sub byte [rsi], 1
jmp LS_388
LE_388:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_389:
jz LE_389
add rsi, 9
jmp LS_389
LE_389:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_386
LE_386:
jmp LS_384
LE_384:
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_390:
jz LE_390
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
add rsi, 2
jmp LS_390
LE_390:
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_391:
jz LE_391
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_392:
jz LE_392
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_392
LE_392:
sub rsi, 1
sub rsi, 2
cmp byte [rsi], 0
LS_393:
jz LE_393
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 12
cmp byte [rsi], 0
LS_394:
jz LE_394
sub rsi, 9
jmp LS_394
LE_394:
add rsi, 3
cmp byte [rsi], 0
LS_395:
jz LE_395
sub byte [rsi], 1
jmp LS_395
LE_395:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_396:
jz LE_396
add rsi, 9
jmp LS_396
LE_396:
add rsi, 1
cmp byte [rsi], 0
LS_397:
jz LE_397
sub byte [rsi], 1
jmp LS_397
LE_397:
add byte [rsi], 1
sub rsi, 1
jmp LS_393
LE_393:
jmp LS_391
LE_391:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_398:
jz LE_398
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_399:
jz LE_399
add rsi, 9
jmp LS_399
LE_399:
sub rsi, 1
sub rsi, 7
jmp LS_398
LE_398:
add rsi, 8
jmp LS_382
LE_382:
sub rsi, 9
cmp byte [rsi], 0
LS_400:
jz LE_400
sub rsi, 9
jmp LS_400
LE_400:
add rsi, 3
cmp byte [rsi], 0
LS_401:
jz LE_401
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_401
LE_401:
sub rsi, 3
cmp byte [rsi], 0
LS_402:
jz LE_402
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_403:
jz LE_403
add rsi, 1
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_404:
jz LE_404
sub byte [rsi], 1
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
jmp LS_404
LE_404:
sub rsi, 1
cmp byte [rsi], 0
LS_405:
jz LE_405
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_405
LE_405:
add rsi, 8
jmp LS_403
LE_403:
sub rsi, 8
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_406:
jz LE_406
add rsi, 1
cmp byte [rsi], 0
LS_407:
jz LE_407
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_408:
jz LE_408
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 10
cmp byte [rsi], 0
LS_409:
jz LE_409
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_409
LE_409:
add rsi, 1
jmp LS_408
LE_408:
sub rsi, 1
cmp byte [rsi], 0
LS_410:
jz LE_410
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 10
jmp LS_410
LE_410:
sub rsi, 1
jmp LS_407
LE_407:
add rsi, 2
cmp byte [rsi], 0
LS_411:
jz LE_411
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_412:
jz LE_412
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 10
jmp LS_412
LE_412:
add rsi, 1
jmp LS_411
LE_411:
sub rsi, 1
cmp byte [rsi], 0
LS_413:
jz LE_413
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_413
LE_413:
sub rsi, 11
jmp LS_406
LE_406:
add rsi, 5
cmp byte [rsi], 0
LS_414:
jz LE_414
sub byte [rsi], 1
jmp LS_414
LE_414:
add rsi, 2
cmp byte [rsi], 0
LS_415:
jz LE_415
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_415
LE_415:
sub rsi, 7
cmp byte [rsi], 0
LS_416:
jz LE_416
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
sub rsi, 5
jmp LS_416
LE_416:
jmp LS_402
LE_402:
add rsi, 4
cmp byte [rsi], 0
LS_417:
jz LE_417
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
add rsi, 3
jmp LS_417
LE_417:
sub rsi, 4
cmp byte [rsi], 0
LS_418:
jz LE_418
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_419:
jz LE_419
add rsi, 1
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_420:
jz LE_420
sub byte [rsi], 1
sub rsi, 2
sub byte [rsi], 1
add rsi, 2
jmp LS_420
LE_420:
sub rsi, 2
cmp byte [rsi], 0
LS_421:
jz LE_421
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_421
LE_421:
add rsi, 8
jmp LS_419
LE_419:
sub rsi, 8
add byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_422:
jz LE_422
add rsi, 1
cmp byte [rsi], 0
LS_423:
jz LE_423
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_424:
jz LE_424
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 11
cmp byte [rsi], 0
LS_425:
jz LE_425
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_425
LE_425:
sub rsi, 1
jmp LS_424
LE_424:
add rsi, 1
cmp byte [rsi], 0
LS_426:
jz LE_426
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 11
jmp LS_426
LE_426:
sub rsi, 2
jmp LS_423
LE_423:
add rsi, 1
cmp byte [rsi], 0
LS_427:
jz LE_427
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_428:
jz LE_428
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 11
jmp LS_428
LE_428:
sub rsi, 1
jmp LS_427
LE_427:
add rsi, 1
cmp byte [rsi], 0
LS_429:
jz LE_429
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_429
LE_429:
sub rsi, 12
jmp LS_422
LE_422:
jmp LS_418
LE_418:
add rsi, 4
cmp byte [rsi], 0
LS_430:
jz LE_430
sub byte [rsi], 1
jmp LS_430
LE_430:
sub rsi, 4
jmp LS_381
LE_381:
add rsi, 4
cmp byte [rsi], 0
LS_431:
jz LE_431
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 2
add rsi, 2
jmp LS_431
LE_431:
sub rsi, 4
cmp byte [rsi], 0
LS_432:
jz LE_432
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_433:
jz LE_433
sub byte [rsi], 1
jmp LS_433
LE_433:
add rsi, 2
cmp byte [rsi], 0
LS_434:
jz LE_434
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_434
LE_434:
sub rsi, 7
cmp byte [rsi], 0
LS_435:
jz LE_435
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
sub rsi, 5
jmp LS_435
LE_435:
add rsi, 9
cmp byte [rsi], 0
LS_436:
jz LE_436
add rsi, 6
add rsi, 3
jmp LS_436
LE_436:
sub rsi, 9
cmp byte [rsi], 0
LS_437:
jz LE_437
add rsi, 1
cmp byte [rsi], 0
LS_438:
jz LE_438
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_439:
jz LE_439
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 11
cmp byte [rsi], 0
LS_440:
jz LE_440
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_440
LE_440:
sub rsi, 1
jmp LS_439
LE_439:
add rsi, 1
cmp byte [rsi], 0
LS_441:
jz LE_441
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
sub rsi, 8
sub rsi, 5
add byte [rsi], 1
add rsi, 11
jmp LS_441
LE_441:
sub rsi, 2
jmp LS_438
LE_438:
add rsi, 1
cmp byte [rsi], 0
LS_442:
jz LE_442
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_443:
jz LE_443
sub byte [rsi], 1
add rsi, 2
sub byte [rsi], 1
sub rsi, 13
add byte [rsi], 1
add rsi, 11
jmp LS_443
LE_443:
sub rsi, 1
jmp LS_442
LE_442:
add rsi, 1
cmp byte [rsi], 0
LS_444:
jz LE_444
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_444
LE_444:
sub rsi, 8
sub rsi, 4
jmp LS_437
LE_437:
jmp LS_432
LE_432:
add rsi, 9
cmp byte [rsi], 0
LS_445:
jz LE_445
add rsi, 2
cmp byte [rsi], 0
LS_446:
jz LE_446
sub byte [rsi], 1
jmp LS_446
LE_446:
add rsi, 1
cmp byte [rsi], 0
LS_447:
jz LE_447
sub byte [rsi], 1
jmp LS_447
LE_447:
add rsi, 6
jmp LS_445
LE_445:
sub rsi, 9
cmp byte [rsi], 0
LS_448:
jz LE_448
sub rsi, 9
jmp LS_448
LE_448:
add rsi, 3
cmp byte [rsi], 0
LS_449:
jz LE_449
sub byte [rsi], 1
jmp LS_449
LE_449:
add rsi, 1
cmp byte [rsi], 0
LS_450:
jz LE_450
sub byte [rsi], 1
jmp LS_450
LE_450:
add rsi, 5
cmp byte [rsi], 0
LS_451:
jz LE_451
add rsi, 5
cmp byte [rsi], 0
LS_452:
jz LE_452
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_452
LE_452:
sub rsi, 4
cmp byte [rsi], 0
LS_453:
jz LE_453
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 1
jmp LS_453
LE_453:
add rsi, 8
jmp LS_451
LE_451:
sub rsi, 9
cmp byte [rsi], 0
LS_454:
jz LE_454
sub rsi, 9
jmp LS_454
LE_454:
add rsi, 9
cmp byte [rsi], 0
LS_455:
jz LE_455
add rsi, 6
cmp byte [rsi], 0
LS_456:
jz LE_456
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_456
LE_456:
sub rsi, 5
cmp byte [rsi], 0
LS_457:
jz LE_457
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
sub rsi, 2
jmp LS_457
LE_457:
add rsi, 8
jmp LS_455
LE_455:
sub rsi, 9
cmp byte [rsi], 0
LS_458:
jz LE_458
sub rsi, 9
jmp LS_458
LE_458:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_459:
jz LE_459
cmp byte [rsi], 0
LS_460:
jz LE_460
add rsi, 4
add rsi, 5
jmp LS_460
LE_460:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_461:
jz LE_461
sub byte [rsi], 1
jmp LS_461
LE_461:
add rsi, 1
cmp byte [rsi], 0
LS_462:
jz LE_462
sub byte [rsi], 1
jmp LS_462
LE_462:
add rsi, 1
cmp byte [rsi], 0
LS_463:
jz LE_463
sub byte [rsi], 1
jmp LS_463
LE_463:
add rsi, 1
cmp byte [rsi], 0
LS_464:
jz LE_464
sub byte [rsi], 1
jmp LS_464
LE_464:
add rsi, 1
cmp byte [rsi], 0
LS_465:
jz LE_465
sub byte [rsi], 1
jmp LS_465
LE_465:
add rsi, 1
cmp byte [rsi], 0
LS_466:
jz LE_466
sub byte [rsi], 1
jmp LS_466
LE_466:
add rsi, 1
cmp byte [rsi], 0
LS_467:
jz LE_467
sub byte [rsi], 1
jmp LS_467
LE_467:
add rsi, 1
cmp byte [rsi], 0
LS_468:
jz LE_468
sub byte [rsi], 1
jmp LS_468
LE_468:
add rsi, 1
cmp byte [rsi], 0
LS_469:
jz LE_469
sub byte [rsi], 1
jmp LS_469
LE_469:
sub rsi, 9
cmp byte [rsi], 0
LS_470:
jz LE_470
sub rsi, 9
jmp LS_470
LE_470:
add rsi, 9
sub byte [rsi], 1
jmp LS_459
LE_459:
add byte [rsi], 1
cmp byte [rsi], 0
LS_471:
jz LE_471
add rsi, 1
add byte [rsi], 1
add rsi, 2
add rsi, 6
jmp LS_471
LE_471:
sub rsi, 9
cmp byte [rsi], 0
LS_472:
jz LE_472
sub rsi, 9
jmp LS_472
LE_472:
add rsi, 9
cmp byte [rsi], 0
LS_473:
jz LE_473
add rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_474:
jz LE_474
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_474
LE_474:
sub rsi, 4
cmp byte [rsi], 0
LS_475:
jz LE_475
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_476:
jz LE_476
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_477:
jz LE_477
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_477
LE_477:
sub rsi, 2
cmp byte [rsi], 0
LS_478:
jz LE_478
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 4
jmp LS_478
LE_478:
add byte [rsi], 1
add rsi, 9
jmp LS_476
LE_476:
sub rsi, 8
cmp byte [rsi], 0
LS_479:
jz LE_479
sub rsi, 9
jmp LS_479
LE_479:
jmp LS_475
LE_475:
add rsi, 9
cmp byte [rsi], 0
LS_480:
jz LE_480
add rsi, 9
jmp LS_480
LE_480:
sub rsi, 9
cmp byte [rsi], 0
LS_481:
jz LE_481
add rsi, 1
cmp byte [rsi], 0
LS_482:
jz LE_482
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_482
LE_482:
sub rsi, 10
jmp LS_481
LE_481:
add rsi, 1
cmp byte [rsi], 0
LS_483:
jz LE_483
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_483
LE_483:
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_473
LE_473:
sub rsi, 9
cmp byte [rsi], 0
LS_484:
jz LE_484
add rsi, 1
cmp byte [rsi], 0
LS_485:
jz LE_485
sub byte [rsi], 1
jmp LS_485
LE_485:
sub rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_486:
jz LE_486
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_487:
jz LE_487
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_487
LE_487:
sub rsi, 1
cmp byte [rsi], 0
LS_488:
jz LE_488
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_488
LE_488:
add rsi, 4
jmp LS_486
LE_486:
sub rsi, 3
cmp byte [rsi], 0
LS_489:
jz LE_489
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_489
LE_489:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_484
LE_484:
add rsi, 9
cmp byte [rsi], 0
LS_490:
jz LE_490
add rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_490
LE_490:
sub rsi, 9
cmp byte [rsi], 0
LS_491:
jz LE_491
sub rsi, 9
jmp LS_491
LE_491:
add rsi, 9
cmp byte [rsi], 0
LS_492:
jz LE_492
add rsi, 1
sub byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_493:
jz LE_493
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_493
LE_493:
sub rsi, 5
cmp byte [rsi], 0
LS_494:
jz LE_494
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 4
sub rsi, 2
cmp byte [rsi], 0
LS_495:
jz LE_495
sub byte [rsi], 1
add rsi, 3
cmp byte [rsi], 0
LS_496:
jz LE_496
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_496
LE_496:
sub rsi, 3
cmp byte [rsi], 0
LS_497:
jz LE_497
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 4
jmp LS_497
LE_497:
add byte [rsi], 1
add rsi, 9
jmp LS_495
LE_495:
sub rsi, 8
cmp byte [rsi], 0
LS_498:
jz LE_498
sub rsi, 9
jmp LS_498
LE_498:
jmp LS_494
LE_494:
add rsi, 9
cmp byte [rsi], 0
LS_499:
jz LE_499
add rsi, 6
add rsi, 3
jmp LS_499
LE_499:
sub rsi, 9
cmp byte [rsi], 0
LS_500:
jz LE_500
add rsi, 2
cmp byte [rsi], 0
LS_501:
jz LE_501
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_501
LE_501:
sub rsi, 11
jmp LS_500
LE_500:
add rsi, 2
cmp byte [rsi], 0
LS_502:
jz LE_502
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_502
LE_502:
sub rsi, 2
add byte [rsi], 1
add rsi, 3
add rsi, 5
jmp LS_492
LE_492:
sub rsi, 9
cmp byte [rsi], 0
LS_503:
jz LE_503
add rsi, 1
cmp byte [rsi], 0
LS_504:
jz LE_504
sub byte [rsi], 1
jmp LS_504
LE_504:
sub rsi, 1
sub byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_505:
jz LE_505
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_506:
jz LE_506
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_506
LE_506:
sub rsi, 1
cmp byte [rsi], 0
LS_507:
jz LE_507
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_507
LE_507:
add rsi, 4
jmp LS_505
LE_505:
sub rsi, 3
cmp byte [rsi], 0
LS_508:
jz LE_508
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 3
jmp LS_508
LE_508:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_503
LE_503:
add rsi, 9
cmp byte [rsi], 0
LS_509:
jz LE_509
add rsi, 4
cmp byte [rsi], 0
LS_510:
jz LE_510
sub byte [rsi], 1
sub rsi, 36
add byte [rsi], 1
add rsi, 17
add rsi, 19
jmp LS_510
LE_510:
add rsi, 5
jmp LS_509
LE_509:
sub rsi, 9
cmp byte [rsi], 0
LS_511:
jz LE_511
sub rsi, 9
jmp LS_511
LE_511:
add rsi, 9
add byte [rsi], 15
cmp byte [rsi], 0
LS_512:
jz LE_512
cmp byte [rsi], 0
LS_513:
jz LE_513
add rsi, 8
add rsi, 1
jmp LS_513
LE_513:
sub rsi, 9
sub byte [rsi], 1
sub rsi, 9
cmp byte [rsi], 0
LS_514:
jz LE_514
sub rsi, 9
jmp LS_514
LE_514:
add rsi, 9
sub byte [rsi], 1
jmp LS_512
LE_512:
add byte [rsi], 1
add rsi, 21
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_515:
jz LE_515
sub rsi, 9
jmp LS_515
LE_515:
add rsi, 9
cmp byte [rsi], 0
LS_516:
jz LE_516
add rsi, 3
cmp byte [rsi], 0
LS_517:
jz LE_517
sub byte [rsi], 1
sub rsi, 3
sub byte [rsi], 1
add rsi, 3
jmp LS_517
LE_517:
add byte [rsi], 1
sub rsi, 3
cmp byte [rsi], 0
LS_518:
jz LE_518
sub byte [rsi], 1
add rsi, 3
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_519:
jz LE_519
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_519
LE_519:
sub rsi, 4
cmp byte [rsi], 0
LS_520:
jz LE_520
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 13
cmp byte [rsi], 0
LS_521:
jz LE_521
sub rsi, 9
jmp LS_521
LE_521:
add rsi, 4
cmp byte [rsi], 0
LS_522:
jz LE_522
sub byte [rsi], 1
jmp LS_522
LE_522:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_523:
jz LE_523
add rsi, 9
jmp LS_523
LE_523:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_520
LE_520:
jmp LS_518
LE_518:
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_524:
jz LE_524
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_524
LE_524:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_525:
jz LE_525
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_526:
jz LE_526
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_526
LE_526:
sub rsi, 3
cmp byte [rsi], 0
LS_527:
jz LE_527
sub byte [rsi], 1
add rsi, 3
add byte [rsi], 1
sub rsi, 1
sub rsi, 11
cmp byte [rsi], 0
LS_528:
jz LE_528
sub rsi, 9
jmp LS_528
LE_528:
add rsi, 3
cmp byte [rsi], 0
LS_529:
jz LE_529
sub byte [rsi], 1
jmp LS_529
LE_529:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_530:
jz LE_530
add rsi, 9
jmp LS_530
LE_530:
add rsi, 1
cmp byte [rsi], 0
LS_531:
jz LE_531
sub byte [rsi], 1
jmp LS_531
LE_531:
add byte [rsi], 1
sub rsi, 1
jmp LS_527
LE_527:
jmp LS_525
LE_525:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_532:
jz LE_532
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_533:
jz LE_533
add rsi, 9
jmp LS_533
LE_533:
sub rsi, 8
jmp LS_532
LE_532:
add rsi, 1
add rsi, 7
jmp LS_516
LE_516:
sub rsi, 9
cmp byte [rsi], 0
LS_534:
jz LE_534
sub rsi, 9
jmp LS_534
LE_534:
add rsi, 2
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_535:
jz LE_535
sub byte [rsi], 1
sub rsi, 4
add byte [rsi], 1
add rsi, 4
jmp LS_535
LE_535:
sub rsi, 4
cmp byte [rsi], 0
LS_536:
jz LE_536
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_537:
jz LE_537
sub byte [rsi], 1
jmp LS_537
LE_537:
sub rsi, 2
jmp LS_536
LE_536:
add rsi, 2
jmp LS_115
LE_115:
sub rsi, 2
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_538:
jz LE_538
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_538
LE_538:
add byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_539:
jz LE_539
sub byte [rsi], 1
add rsi, 4
sub byte [rsi], 1
sub rsi, 6
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 2
jmp LS_539
LE_539:
add rsi, 4
cmp byte [rsi], 0
LS_540:
jz LE_540
sub byte [rsi], 1
sub rsi, 7
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 7
jmp LS_540
LE_540:
sub rsi, 3
cmp byte [rsi], 0
LS_541:
jz LE_541
sub byte [rsi], 1
jmp LS_541
LE_541:
add rsi, 1
cmp byte [rsi], 0
LS_542:
jz LE_542
sub byte [rsi], 1
jmp LS_542
LE_542:
add rsi, 1
cmp byte [rsi], 0
LS_543:
jz LE_543
sub byte [rsi], 1
jmp LS_543
LE_543:
add rsi, 1
cmp byte [rsi], 0
LS_544:
jz LE_544
sub byte [rsi], 1
jmp LS_544
LE_544:
add rsi, 1
cmp byte [rsi], 0
LS_545:
jz LE_545
sub byte [rsi], 1
jmp LS_545
LE_545:
add rsi, 1
cmp byte [rsi], 0
LS_546:
jz LE_546
sub byte [rsi], 1
jmp LS_546
LE_546:
add rsi, 3
cmp byte [rsi], 0
LS_547:
jz LE_547
add rsi, 1
cmp byte [rsi], 0
LS_548:
jz LE_548
sub byte [rsi], 1
jmp LS_548
LE_548:
add rsi, 1
cmp byte [rsi], 0
LS_549:
jz LE_549
sub byte [rsi], 1
jmp LS_549
LE_549:
add rsi, 1
cmp byte [rsi], 0
LS_550:
jz LE_550
sub byte [rsi], 1
jmp LS_550
LE_550:
add rsi, 1
cmp byte [rsi], 0
LS_551:
jz LE_551
sub byte [rsi], 1
jmp LS_551
LE_551:
add rsi, 1
cmp byte [rsi], 0
LS_552:
jz LE_552
sub byte [rsi], 1
jmp LS_552
LE_552:
add rsi, 1
cmp byte [rsi], 0
LS_553:
jz LE_553
sub byte [rsi], 1
jmp LS_553
LE_553:
add rsi, 3
jmp LS_547
LE_547:
sub rsi, 9
cmp byte [rsi], 0
LS_554:
jz LE_554
sub rsi, 9
jmp LS_554
LE_554:
add rsi, 9
cmp byte [rsi], 0
LS_555:
jz LE_555
add rsi, 5
cmp byte [rsi], 0
LS_556:
jz LE_556
sub byte [rsi], 1
jmp LS_556
LE_556:
add rsi, 4
jmp LS_555
LE_555:
sub rsi, 9
cmp byte [rsi], 0
LS_557:
jz LE_557
sub rsi, 9
jmp LS_557
LE_557:
add rsi, 1
add byte [rsi], 10
cmp byte [rsi], 0
LS_558:
jz LE_558
sub byte [rsi], 1
cmp byte [rsi], 0
LS_559:
jz LE_559
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_559
LE_559:
add rsi, 9
jmp LS_558
LE_558:
add rsi, 4
add byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
sub rsi, 5
cmp byte [rsi], 0
LS_560:
jz LE_560
sub rsi, 9
jmp LS_560
LE_560:
add rsi, 7
cmp byte [rsi], 0
LS_561:
jz LE_561
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_561
LE_561:
sub rsi, 7
cmp byte [rsi], 0
LS_562:
jz LE_562
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
cmp byte [rsi], 0
LS_563:
jz LE_563
sub byte [rsi], 1
jmp LS_563
LE_563:
add rsi, 2
cmp byte [rsi], 0
LS_564:
jz LE_564
add rsi, 9
jmp LS_564
LE_564:
sub rsi, 6
sub rsi, 3
cmp byte [rsi], 0
LS_565:
jz LE_565
add rsi, 7
cmp byte [rsi], 0
LS_566:
jz LE_566
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_566
LE_566:
sub rsi, 6
cmp byte [rsi], 0
LS_567:
jz LE_567
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 7
cmp byte [rsi], 0
LS_568:
jz LE_568
sub rsi, 9
jmp LS_568
LE_568:
add rsi, 7
cmp byte [rsi], 0
LS_569:
jz LE_569
sub byte [rsi], 1
jmp LS_569
LE_569:
add byte [rsi], 1
add rsi, 3
jmp LS_567
LE_567:
sub rsi, 5
sub rsi, 5
jmp LS_565
LE_565:
jmp LS_562
LE_562:
add rsi, 7
cmp byte [rsi], 0
LS_570:
jz LE_570
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_570
LE_570:
sub rsi, 7
cmp byte [rsi], 0
LS_571:
jz LE_571
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_572:
jz LE_572
add rsi, 1
add byte [rsi], 1
add rsi, 4
cmp byte [rsi], 0
LS_573:
jz LE_573
sub byte [rsi], 1
sub rsi, 4
sub byte [rsi], 1
add rsi, 4
jmp LS_573
LE_573:
sub rsi, 4
cmp byte [rsi], 0
LS_574:
jz LE_574
sub byte [rsi], 1
add rsi, 4
add byte [rsi], 1
sub rsi, 4
jmp LS_574
LE_574:
add rsi, 8
jmp LS_572
LE_572:
sub rsi, 2
add byte [rsi], 1
sub rsi, 7
cmp byte [rsi], 0
LS_575:
jz LE_575
add rsi, 5
cmp byte [rsi], 0
LS_576:
jz LE_576
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_576
LE_576:
sub rsi, 14
jmp LS_575
LE_575:
add rsi, 9
cmp byte [rsi], 0
LS_577:
jz LE_577
add rsi, 9
jmp LS_577
LE_577:
sub rsi, 6
sub rsi, 3
cmp byte [rsi], 0
LS_578:
jz LE_578
add rsi, 1
cmp byte [rsi], 0
LS_579:
jz LE_579
sub byte [rsi], 1
jmp LS_579
LE_579:
sub rsi, 1
sub byte [rsi], 1
add rsi, 7
cmp byte [rsi], 0
LS_580:
jz LE_580
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_581:
jz LE_581
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_581
LE_581:
sub rsi, 1
cmp byte [rsi], 0
LS_582:
jz LE_582
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_582
LE_582:
add rsi, 7
jmp LS_580
LE_580:
sub rsi, 6
cmp byte [rsi], 0
LS_583:
jz LE_583
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 6
jmp LS_583
LE_583:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_578
LE_578:
add rsi, 7
sub byte [rsi], 1
sub rsi, 4
cmp byte [rsi], 0
LS_584:
jz LE_584
sub byte [rsi], 1
jmp LS_584
LE_584:
add byte [rsi], 1
sub rsi, 3
jmp LS_571
LE_571:
add byte [rsi], 1
add rsi, 7
cmp byte [rsi], 0
LS_585:
jz LE_585
sub byte [rsi], 1
sub rsi, 7
sub byte [rsi], 1
add rsi, 7
jmp LS_585
LE_585:
add byte [rsi], 1
sub rsi, 7
cmp byte [rsi], 0
LS_586:
jz LE_586
sub byte [rsi], 1
add rsi, 7
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_587:
jz LE_587
add rsi, 3
add rsi, 2
cmp byte [rsi], 0
LS_588:
jz LE_588
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_588
LE_588:
add rsi, 4
jmp LS_587
LE_587:
sub rsi, 9
cmp byte [rsi], 0
LS_589:
jz LE_589
add rsi, 1
cmp byte [rsi], 0
LS_590:
jz LE_590
sub byte [rsi], 1
jmp LS_590
LE_590:
sub rsi, 1
sub byte [rsi], 1
add rsi, 7
cmp byte [rsi], 0
LS_591:
jz LE_591
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_592:
jz LE_592
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 3
add byte [rsi], 1
add rsi, 3
jmp LS_592
LE_592:
sub rsi, 1
cmp byte [rsi], 0
LS_593:
jz LE_593
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_593
LE_593:
add rsi, 7
jmp LS_591
LE_591:
sub rsi, 3
sub rsi, 3
cmp byte [rsi], 0
LS_594:
jz LE_594
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 6
jmp LS_594
LE_594:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_589
LE_589:
add rsi, 1
add byte [rsi], 5
cmp byte [rsi], 0
LS_595:
jz LE_595
sub byte [rsi], 1
cmp byte [rsi], 0
LS_596:
jz LE_596
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_596
LE_596:
add rsi, 9
jmp LS_595
LE_595:
add rsi, 4
add byte [rsi], 1
sub rsi, 4
sub rsi, 1
cmp byte [rsi], 0
LS_597:
jz LE_597
sub rsi, 9
jmp LS_597
LE_597:
add rsi, 9
cmp byte [rsi], 0
LS_598:
jz LE_598
add rsi, 5
cmp byte [rsi], 0
LS_599:
jz LE_599
sub byte [rsi], 1
sub rsi, 5
sub byte [rsi], 1
add rsi, 5
jmp LS_599
LE_599:
add byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_600:
jz LE_600
sub byte [rsi], 1
add rsi, 5
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_601:
jz LE_601
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_601
LE_601:
sub rsi, 5
sub rsi, 2
cmp byte [rsi], 0
LS_602:
jz LE_602
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 16
cmp byte [rsi], 0
LS_603:
jz LE_603
sub rsi, 9
jmp LS_603
LE_603:
add rsi, 4
cmp byte [rsi], 0
LS_604:
jz LE_604
sub byte [rsi], 1
jmp LS_604
LE_604:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_605:
jz LE_605
add rsi, 9
jmp LS_605
LE_605:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_602
LE_602:
jmp LS_600
LE_600:
add byte [rsi], 1
add rsi, 7
cmp byte [rsi], 0
LS_606:
jz LE_606
sub byte [rsi], 1
sub rsi, 2
sub rsi, 5
sub byte [rsi], 1
add rsi, 7
jmp LS_606
LE_606:
add byte [rsi], 1
sub rsi, 7
cmp byte [rsi], 0
LS_607:
jz LE_607
sub byte [rsi], 1
add rsi, 7
sub byte [rsi], 1
sub rsi, 2
cmp byte [rsi], 0
LS_608:
jz LE_608
sub byte [rsi], 1
sub rsi, 5
add byte [rsi], 1
add rsi, 5
jmp LS_608
LE_608:
sub rsi, 5
cmp byte [rsi], 0
LS_609:
jz LE_609
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 14
cmp byte [rsi], 0
LS_610:
jz LE_610
sub rsi, 4
sub rsi, 5
jmp LS_610
LE_610:
add rsi, 3
cmp byte [rsi], 0
LS_611:
jz LE_611
sub byte [rsi], 1
jmp LS_611
LE_611:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_612:
jz LE_612
add rsi, 9
jmp LS_612
LE_612:
add rsi, 1
cmp byte [rsi], 0
LS_613:
jz LE_613
sub byte [rsi], 1
jmp LS_613
LE_613:
add byte [rsi], 1
sub rsi, 1
jmp LS_609
LE_609:
jmp LS_607
LE_607:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_614:
jz LE_614
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_615:
jz LE_615
add rsi, 9
jmp LS_615
LE_615:
sub rsi, 8
jmp LS_614
LE_614:
add rsi, 8
jmp LS_598
LE_598:
sub rsi, 8
sub rsi, 1
cmp byte [rsi], 0
LS_616:
jz LE_616
sub rsi, 9
jmp LS_616
LE_616:
add rsi, 4
cmp byte [rsi], 0
LS_617:
jz LE_617
sub byte [rsi], 1
jmp LS_617
LE_617:
sub rsi, 3
add byte [rsi], 5
cmp byte [rsi], 0
LS_618:
jz LE_618
sub byte [rsi], 1
cmp byte [rsi], 0
LS_619:
jz LE_619
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_619
LE_619:
add rsi, 9
jmp LS_618
LE_618:
add rsi, 4
sub byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_620:
jz LE_620
sub rsi, 8
sub rsi, 1
jmp LS_620
LE_620:
jmp LS_586
LE_586:
add rsi, 3
jmp LS_23
LE_23:
sub rsi, 4
mov rax, 1
mov rdi, 1
mov rdx, 1
syscall
add rsi, 10
cmp byte [rsi], 0
LS_621:
jz LE_621
add rsi, 6
cmp byte [rsi], 0
LS_622:
jz LE_622
sub byte [rsi], 1
jmp LS_622
LE_622:
add rsi, 3
jmp LS_621
LE_621:
sub rsi, 9
cmp byte [rsi], 0
LS_623:
jz LE_623
sub rsi, 9
jmp LS_623
LE_623:
add rsi, 1
add byte [rsi], 9
cmp byte [rsi], 0
LS_624:
jz LE_624
sub byte [rsi], 1
cmp byte [rsi], 0
LS_625:
jz LE_625
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_625
LE_625:
add rsi, 9
jmp LS_624
LE_624:
add rsi, 5
add byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 15
cmp byte [rsi], 0
LS_626:
jz LE_626
sub rsi, 9
jmp LS_626
LE_626:
add rsi, 8
cmp byte [rsi], 0
LS_627:
jz LE_627
sub byte [rsi], 1
sub rsi, 8
add byte [rsi], 1
add rsi, 8
jmp LS_627
LE_627:
sub rsi, 8
cmp byte [rsi], 0
LS_628:
jz LE_628
sub byte [rsi], 1
add rsi, 8
add byte [rsi], 1
cmp byte [rsi], 0
LS_629:
jz LE_629
sub byte [rsi], 1
jmp LS_629
LE_629:
add rsi, 1
cmp byte [rsi], 0
LS_630:
jz LE_630
add rsi, 9
jmp LS_630
LE_630:
sub rsi, 9
cmp byte [rsi], 0
LS_631:
jz LE_631
add rsi, 8
cmp byte [rsi], 0
LS_632:
jz LE_632
sub byte [rsi], 1
sub rsi, 7
add byte [rsi], 1
add rsi, 7
jmp LS_632
LE_632:
sub rsi, 7
cmp byte [rsi], 0
LS_633:
jz LE_633
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 8
cmp byte [rsi], 0
LS_634:
jz LE_634
sub rsi, 9
jmp LS_634
LE_634:
add rsi, 8
cmp byte [rsi], 0
LS_635:
jz LE_635
sub byte [rsi], 1
jmp LS_635
LE_635:
add byte [rsi], 1
add rsi, 2
jmp LS_633
LE_633:
sub rsi, 10
jmp LS_631
LE_631:
jmp LS_628
LE_628:
add rsi, 8
cmp byte [rsi], 0
LS_636:
jz LE_636
sub byte [rsi], 1
sub rsi, 7
sub rsi, 1
add byte [rsi], 1
add rsi, 8
jmp LS_636
LE_636:
sub rsi, 8
cmp byte [rsi], 0
LS_637:
jz LE_637
sub byte [rsi], 1
add rsi, 8
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_638:
jz LE_638
add rsi, 1
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_639:
jz LE_639
sub byte [rsi], 1
sub rsi, 5
sub byte [rsi], 1
add rsi, 5
jmp LS_639
LE_639:
sub rsi, 5
cmp byte [rsi], 0
LS_640:
jz LE_640
sub byte [rsi], 1
add rsi, 5
add byte [rsi], 1
sub rsi, 5
jmp LS_640
LE_640:
add rsi, 8
jmp LS_638
LE_638:
sub rsi, 1
add byte [rsi], 1
sub rsi, 8
cmp byte [rsi], 0
LS_641:
jz LE_641
add rsi, 6
cmp byte [rsi], 0
LS_642:
jz LE_642
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_642
LE_642:
sub rsi, 15
jmp LS_641
LE_641:
add rsi, 9
cmp byte [rsi], 0
LS_643:
jz LE_643
add rsi, 9
jmp LS_643
LE_643:
sub rsi, 9
cmp byte [rsi], 0
LS_644:
jz LE_644
add rsi, 1
cmp byte [rsi], 0
LS_645:
jz LE_645
sub byte [rsi], 1
jmp LS_645
LE_645:
sub rsi, 1
sub byte [rsi], 1
add rsi, 2
add rsi, 6
cmp byte [rsi], 0
LS_646:
jz LE_646
sub byte [rsi], 1
sub rsi, 8
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_647:
jz LE_647
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_647
LE_647:
sub rsi, 1
cmp byte [rsi], 0
LS_648:
jz LE_648
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_648
LE_648:
add rsi, 8
jmp LS_646
LE_646:
sub rsi, 7
cmp byte [rsi], 0
LS_649:
jz LE_649
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 7
jmp LS_649
LE_649:
sub rsi, 1
add byte [rsi], 1
sub rsi, 8
sub rsi, 1
jmp LS_644
LE_644:
add rsi, 8
sub byte [rsi], 1
sub rsi, 5
cmp byte [rsi], 0
LS_650:
jz LE_650
sub byte [rsi], 1
jmp LS_650
LE_650:
add byte [rsi], 1
sub rsi, 3
jmp LS_637
LE_637:
add byte [rsi], 1
add rsi, 8
cmp byte [rsi], 0
LS_651:
jz LE_651
sub byte [rsi], 1
sub rsi, 8
sub byte [rsi], 1
add rsi, 8
jmp LS_651
LE_651:
add byte [rsi], 1
sub rsi, 8
cmp byte [rsi], 0
LS_652:
jz LE_652
sub byte [rsi], 1
add rsi, 8
sub byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_653:
jz LE_653
add rsi, 5
add rsi, 1
cmp byte [rsi], 0
LS_654:
jz LE_654
sub byte [rsi], 1
add rsi, 2
add byte [rsi], 1
sub rsi, 2
jmp LS_654
LE_654:
add rsi, 3
jmp LS_653
LE_653:
sub rsi, 9
cmp byte [rsi], 0
LS_655:
jz LE_655
add rsi, 1
cmp byte [rsi], 0
LS_656:
jz LE_656
sub byte [rsi], 1
jmp LS_656
LE_656:
sub rsi, 1
sub byte [rsi], 1
add rsi, 8
cmp byte [rsi], 0
LS_657:
jz LE_657
sub byte [rsi], 1
sub rsi, 8
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_658:
jz LE_658
sub rsi, 1
sub byte [rsi], 1
add rsi, 1
sub byte [rsi], 1
sub rsi, 2
add byte [rsi], 1
add rsi, 2
jmp LS_658
LE_658:
sub rsi, 1
cmp byte [rsi], 0
LS_659:
jz LE_659
sub byte [rsi], 1
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_659
LE_659:
add rsi, 8
jmp LS_657
LE_657:
sub rsi, 4
sub rsi, 3
cmp byte [rsi], 0
LS_660:
jz LE_660
sub byte [rsi], 1
add rsi, 7
add byte [rsi], 1
sub rsi, 7
jmp LS_660
LE_660:
sub rsi, 1
add byte [rsi], 1
sub rsi, 9
jmp LS_655
LE_655:
add rsi, 1
add byte [rsi], 5
cmp byte [rsi], 0
LS_661:
jz LE_661
sub byte [rsi], 1
cmp byte [rsi], 0
LS_662:
jz LE_662
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_662
LE_662:
add rsi, 9
jmp LS_661
LE_661:
add rsi, 5
add byte [rsi], 1
add rsi, 1
add rsi, 26
add byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_663:
jz LE_663
sub rsi, 9
jmp LS_663
LE_663:
add rsi, 9
cmp byte [rsi], 0
LS_664:
jz LE_664
add rsi, 6
cmp byte [rsi], 0
LS_665:
jz LE_665
sub byte [rsi], 1
sub rsi, 6
sub byte [rsi], 1
add rsi, 6
jmp LS_665
LE_665:
add byte [rsi], 1
sub rsi, 3
sub rsi, 3
cmp byte [rsi], 0
LS_666:
jz LE_666
sub byte [rsi], 1
add rsi, 6
sub byte [rsi], 1
add rsi, 2
cmp byte [rsi], 0
LS_667:
jz LE_667
sub byte [rsi], 1
sub rsi, 8
add byte [rsi], 1
add rsi, 8
jmp LS_667
LE_667:
sub rsi, 8
cmp byte [rsi], 0
LS_668:
jz LE_668
sub byte [rsi], 1
add rsi, 8
add byte [rsi], 1
sub rsi, 17
cmp byte [rsi], 0
LS_669:
jz LE_669
sub rsi, 9
jmp LS_669
LE_669:
add rsi, 4
cmp byte [rsi], 0
LS_670:
jz LE_670
sub byte [rsi], 1
jmp LS_670
LE_670:
add byte [rsi], 1
add rsi, 5
cmp byte [rsi], 0
LS_671:
jz LE_671
add rsi, 9
jmp LS_671
LE_671:
add rsi, 1
add byte [rsi], 1
sub rsi, 1
jmp LS_668
LE_668:
jmp LS_666
LE_666:
add byte [rsi], 1
add rsi, 8
cmp byte [rsi], 0
LS_672:
jz LE_672
sub byte [rsi], 1
sub rsi, 8
sub byte [rsi], 1
add rsi, 8
jmp LS_672
LE_672:
add byte [rsi], 1
sub rsi, 8
cmp byte [rsi], 0
LS_673:
jz LE_673
sub byte [rsi], 1
add rsi, 8
sub byte [rsi], 1
sub rsi, 1
sub rsi, 1
cmp byte [rsi], 0
LS_674:
jz LE_674
sub byte [rsi], 1
sub rsi, 6
add byte [rsi], 1
add rsi, 6
jmp LS_674
LE_674:
sub rsi, 6
cmp byte [rsi], 0
LS_675:
jz LE_675
sub byte [rsi], 1
add rsi, 6
add byte [rsi], 1
sub rsi, 15
cmp byte [rsi], 0
LS_676:
jz LE_676
sub rsi, 9
jmp LS_676
LE_676:
add rsi, 3
cmp byte [rsi], 0
LS_677:
jz LE_677
sub byte [rsi], 1
jmp LS_677
LE_677:
add byte [rsi], 1
add rsi, 6
cmp byte [rsi], 0
LS_678:
jz LE_678
add rsi, 8
add rsi, 1
jmp LS_678
LE_678:
add rsi, 1
cmp byte [rsi], 0
LS_679:
jz LE_679
sub byte [rsi], 1
jmp LS_679
LE_679:
add byte [rsi], 1
sub rsi, 1
jmp LS_675
LE_675:
jmp LS_673
LE_673:
add byte [rsi], 1
add rsi, 1
cmp byte [rsi], 0
LS_680:
jz LE_680
sub byte [rsi], 1
sub rsi, 1
cmp byte [rsi], 0
LS_681:
jz LE_681
add rsi, 9
jmp LS_681
LE_681:
sub rsi, 8
jmp LS_680
LE_680:
add rsi, 8
jmp LS_664
LE_664:
sub rsi, 9
cmp byte [rsi], 0
LS_682:
jz LE_682
sub rsi, 9
jmp LS_682
LE_682:
add rsi, 4
cmp byte [rsi], 0
LS_683:
jz LE_683
sub byte [rsi], 1
jmp LS_683
LE_683:
sub rsi, 3
add byte [rsi], 5
cmp byte [rsi], 0
LS_684:
jz LE_684
sub byte [rsi], 1
cmp byte [rsi], 0
LS_685:
jz LE_685
sub byte [rsi], 1
add rsi, 9
add byte [rsi], 1
sub rsi, 9
jmp LS_685
LE_685:
add rsi, 9
jmp LS_684
LE_684:
add rsi, 5
sub byte [rsi], 1
add rsi, 27
sub byte [rsi], 1
sub rsi, 6
cmp byte [rsi], 0
LS_686:
jz LE_686
sub rsi, 6
sub rsi, 3
jmp LS_686
LE_686:
jmp LS_652
LE_652:
add rsi, 3
jmp LS_13
LE_13:
sub rsi, 3
mov rax, 60
xor rdi, rdi
syscall