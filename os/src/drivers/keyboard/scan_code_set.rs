// scan code set 1
// 主键盘区及功能区
pub const KEY_ESC_PRESSED: [u8; 6] = [0x01, 0, 0, 0, 0, 0];
pub const KEY_ESC_RELEASED: [u8; 6] = [0x81, 0, 0, 0, 0, 0];
pub const KEY_F1_PRESSED: [u8; 6] = [0x3b, 0, 0, 0, 0, 0];
pub const KEY_F1_RELEASED: [u8; 6] = [0xbb, 0, 0, 0, 0, 0];
pub const KEY_F2_PRESSED: [u8; 6] = [0x3c, 0, 0, 0, 0, 0];
pub const KEY_F2_RELEASED: [u8; 6] = [0xbc, 0, 0, 0, 0, 0];
pub const KEY_F3_PRESSED: [u8; 6] = [0x3d, 0, 0, 0, 0, 0];
pub const KEY_F3_RELEASED: [u8; 6] = [0xbd, 0, 0, 0, 0, 0];
pub const KEY_F4_PRESSED: [u8; 6] = [0x3e, 0, 0, 0, 0, 0];
pub const KEY_F4_RELEASED: [u8; 6] = [0xbe, 0, 0, 0, 0, 0];
pub const KEY_F5_PRESSED: [u8; 6] = [0x3f, 0, 0, 0, 0, 0];
pub const KEY_F5_RELEASED: [u8; 6] = [0xbf, 0, 0, 0, 0, 0];
pub const KEY_F6_PRESSED: [u8; 6] = [0x40, 0, 0, 0, 0, 0];
pub const KEY_F6_RELEASED: [u8; 6] = [0xc0, 0, 0, 0, 0, 0];
pub const KEY_F7_PRESSED: [u8; 6] = [0x41, 0, 0, 0, 0, 0];
pub const KEY_F7_RELEASED: [u8; 6] = [0xc1, 0, 0, 0, 0, 0];
pub const KEY_F8_PRESSED: [u8; 6] = [0x42, 0, 0, 0, 0, 0];
pub const KEY_F8_RELEASED: [u8; 6] = [0xc2, 0, 0, 0, 0, 0];
pub const KEY_F9_PRESSED: [u8; 6] = [0x43, 0, 0, 0, 0, 0];
pub const KEY_F9_RELEASED: [u8; 6] = [0xc3, 0, 0, 0, 0, 0];
pub const KEY_F10_PRESSED: [u8; 6] = [0x44, 0, 0, 0, 0, 0];
pub const KEY_F10_RELEASED: [u8; 6] = [0xc4, 0, 0, 0, 0, 0];
pub const KEY_F11_PRESSED: [u8; 6] = [0x57, 0, 0, 0, 0, 0];
pub const KEY_F11_RELEASED: [u8; 6] = [0xd8, 0, 0, 0, 0, 0];
pub const KEY_F12_PRESSED: [u8; 6] = [0x58, 0, 0, 0, 0, 0];
pub const KEY_F12_RELEASED: [u8; 6] = [0xd8, 0, 0, 0, 0, 0];
// ~`
pub const KEY_BACKTICK_PRESSED: [u8; 6] = [0x29, 0, 0, 0, 0, 0];
pub const KEY_BACKTICK_RELEASED: [u8; 6] = [0xa9, 0, 0, 0, 0, 0];
// !1
pub const KEY_1_PRESSED: [u8; 6] = [0x02, 0, 0, 0, 0, 0];
pub const KEY_1_RELEASED: [u8; 6] = [0x82, 0, 0, 0, 0, 0];
// @2
pub const KEY_2_PRESSED: [u8; 6] = [0x03, 0, 0, 0, 0, 0];
pub const KEY_2_RELEASED: [u8; 6] = [0x83, 0, 0, 0, 0, 0];
// #3
pub const KEY_3_PRESSED: [u8; 6] = [0x04, 0, 0, 0, 0, 0];
pub const KEY_3_RELEASED: [u8; 6] = [0x84, 0, 0, 0, 0, 0];
// $4
pub const KEY_4_PRESSED: [u8; 6] = [0x05, 0, 0, 0, 0, 0];
pub const KEY_4_RELEASED: [u8; 6] = [0x85, 0, 0, 0, 0, 0];
// %5
pub const KEY_5_PRESSED: [u8; 6] = [0x06, 0, 0, 0, 0, 0];
pub const KEY_5_RELEASED: [u8; 6] = [0x86, 0, 0, 0, 0, 0];
// ^6
pub const KEY_6_PRESSED: [u8; 6] = [0x07, 0, 0, 0, 0, 0];
pub const KEY_6_RELEASED: [u8; 6] = [0x87, 0, 0, 0, 0, 0];
// &7
pub const KEY_7_PRESSED: [u8; 6] = [0x08, 0, 0, 0, 0, 0];
pub const KEY_7_RELEASED: [u8; 6] = [0x88, 0, 0, 0, 0, 0];
// *8
pub const KEY_8_PRESSED: [u8; 6] = [0x092, 0, 0, 0, 0, 0];
pub const KEY_8_RELEASED: [u8; 6] = [0x89, 0, 0, 0, 0, 0];
// (9
pub const KEY_9_PRESSED: [u8; 6] = [0x0a, 0, 0, 0, 0, 0];
pub const KEY_9_RELEASED: [u8; 6] = [0x8a, 0, 0, 0, 0, 0];
// )0
pub const KEY_0_PRESSED: [u8; 6] = [0x0b, 0, 0, 0, 0, 0];
pub const KEY_0_RELEASED: [u8; 6] = [0x8b, 0, 0, 0, 0, 0];
// _-
pub const KEY_MINUS_PRESSED: [u8; 6] = [0x0c, 0, 0, 0, 0, 0];
pub const KEY_MINUS_RELEASED: [u8; 6] = [0x8c, 0, 0, 0, 0, 0];
// +=
pub const KEY_EQUAL_PRESSED: [u8; 6] = [0x0d, 0, 0, 0, 0, 0];
pub const KEY_EQUAL_RELEASED: [u8; 6] = [0x8d, 0, 0, 0, 0, 0];
pub const KEY_BACKSPACE_PRESSED: [u8; 6] = [0x0e, 0, 0, 0, 0, 0];
pub const KEY_BACKSPACE_RELEASED: [u8; 6] = [0x8e, 0, 0, 0, 0, 0];
pub const KEY_TAB_PRESSED: [u8; 6] = [0x0f, 0, 0, 0, 0, 0];
pub const KEY_TAB_RELEASED: [u8; 6] = [0x8f, 0, 0, 0, 0, 0];
pub const KEY_Q_PRESSED: [u8; 6] = [0x10, 0, 0, 0, 0, 0];
pub const KEY_Q_RELEASED: [u8; 6] = [0x90, 0, 0, 0, 0, 0];
pub const KEY_W_PRESSED: [u8; 6] = [0x11, 0, 0, 0, 0, 0];
pub const KEY_W_RELEASED: [u8; 6] = [0x91, 0, 0, 0, 0, 0];
pub const KEY_E_PRESSED: [u8; 6] = [0x12, 0, 0, 0, 0, 0];
pub const KEY_E_RELEASED: [u8; 6] = [0x92, 0, 0, 0, 0, 0];
pub const KEY_R_PRESSED: [u8; 6] = [0x13, 0, 0, 0, 0, 0];
pub const KEY_R_RELEASED: [u8; 6] = [0x93, 0, 0, 0, 0, 0];
pub const KEY_T_PRESSED: [u8; 6] = [0x14, 0, 0, 0, 0, 0];
pub const KEY_T_RELEASED: [u8; 6] = [0x94, 0, 0, 0, 0, 0];
pub const KEY_Y_PRESSED: [u8; 6] = [0x15, 0, 0, 0, 0, 0];
pub const KEY_Y_RELEASED: [u8; 6] = [0x95, 0, 0, 0, 0, 0];
pub const KEY_U_PRESSED: [u8; 6] = [0x16, 0, 0, 0, 0, 0];
pub const KEY_U_RELEASED: [u8; 6] = [0x96, 0, 0, 0, 0, 0];
pub const KEY_I_PRESSED: [u8; 6] = [0x17, 0, 0, 0, 0, 0];
pub const KEY_I_RELEASED: [u8; 6] = [0x97, 0, 0, 0, 0, 0];
pub const KEY_O_PRESSED: [u8; 6] = [0x18, 0, 0, 0, 0, 0];
pub const KEY_O_RELEASED: [u8; 6] = [0x98, 0, 0, 0, 0, 0];
pub const KEY_P_PRESSED: [u8; 6] = [0x19, 0, 0, 0, 0, 0];
pub const KEY_P_RELEASED: [u8; 6] = [0x99, 0, 0, 0, 0, 0];
// {[
pub const KEY_LEFT_SQUARE_BRACKET_PRESSED: [u8; 6] = [0x1a, 0, 0, 0, 0, 0];
pub const KEY_LEFT_SQUARE_BRACKET_RELEASED: [u8; 6] = [0x9a, 0, 0, 0, 0, 0];
// }]
pub const KEY_RIGHT_SQUARE_BRACKET_PRESSED: [u8; 6] = [0x1b, 0, 0, 0, 0, 0];
pub const KEY_RIGHT_SQUARE_BRACKET_RELEASED: [u8; 6] = [0x9b, 0, 0, 0, 0, 0];
// |\
pub const KEY_BACKSLASH_PRESSED: [u8; 6] = [0x2b, 0, 0, 0, 0, 0];
pub const KEY_BACKSLASH_RELEASED: [u8; 6] = [0xab, 0, 0, 0, 0, 0];
pub const KEY_CAPS_LOCK_PRESSED: [u8; 6] = [0x3a, 0, 0, 0, 0, 0];
pub const KEY_CAPS_LOCK_RELEASED: [u8; 6] = [0xba, 0, 0, 0, 0, 0];
pub const KEY_A_PRESSED: [u8; 6] = [0x1e, 0, 0, 0, 0, 0];
pub const KEY_A_RELEASED: [u8; 6] = [0x9e, 0, 0, 0, 0, 0];
pub const KEY_S_PRESSED: [u8; 6] = [0x1f, 0, 0, 0, 0, 0];
pub const KEY_S_RELEASED: [u8; 6] = [0x9f, 0, 0, 0, 0, 0];
pub const KEY_D_PRESSED: [u8; 6] = [0x20, 0, 0, 0, 0, 0];
pub const KEY_D_RELEASED: [u8; 6] = [0xa0, 0, 0, 0, 0, 0];
pub const KEY_F_PRESSED: [u8; 6] = [0x21, 0, 0, 0, 0, 0];
pub const KEY_F_RELEASED: [u8; 6] = [0xa1, 0, 0, 0, 0, 0];
pub const KEY_G_PRESSED: [u8; 6] = [0x22, 0, 0, 0, 0, 0];
pub const KEY_G_RELEASED: [u8; 6] = [0xa2, 0, 0, 0, 0, 0];
pub const KEY_H_PRESSED: [u8; 6] = [0x23, 0, 0, 0, 0, 0];
pub const KEY_H_RELEASED: [u8; 6] = [0xa3, 0, 0, 0, 0, 0];
pub const KEY_J_PRESSED: [u8; 6] = [0x24, 0, 0, 0, 0, 0];
pub const KEY_J_RELEASED: [u8; 6] = [0xa5, 0, 0, 0, 0, 0];
pub const KEY_K_PRESSED: [u8; 6] = [0x25, 0, 0, 0, 0, 0];
pub const KEY_K_RELEASED: [u8; 6] = [0xa5, 0, 0, 0, 0, 0];
pub const KEY_L_PRESSED: [u8; 6] = [0x26, 0, 0, 0, 0, 0];
pub const KEY_L_RELEASED: [u8; 6] = [0xa6, 0, 0, 0, 0, 0];
// :;
pub const KEY_SEMICOLON_PRESSED: [u8; 6] = [0x27, 0, 0, 0, 0, 0];
pub const KEY_SEMICOLON_RELEASED: [u8; 6] = [0xa7, 0, 0, 0, 0, 0];
// "'
pub const KEY_QUOTE_PRESSED: [u8; 6] = [0x28, 0, 0, 0, 0, 0];
pub const KEY_QUOTE_RELEASED: [u8; 6] = [0xa8, 0, 0, 0, 0, 0];
pub const KEY_ENTER_PRESSED: [u8; 6] = [0x1c, 0, 0, 0, 0, 0];
pub const KEY_ENTER_RELEASED: [u8; 6] = [0x9c, 0, 0, 0, 0, 0];
pub const KEY_LEFT_SHIFT_PRESSED: [u8; 6] = [0x2a, 0, 0, 0, 0, 0];
pub const KEY_LEFT_SHIFT_RELEASED: [u8; 6] = [0xaa, 0, 0, 0, 0, 0];
pub const KEY_Z_PRESSED: [u8; 6] = [0x2c, 0, 0, 0, 0, 0];
pub const KEY_Z_RELEASED: [u8; 6] = [0xac, 0, 0, 0, 0, 0];
pub const KEY_X_PRESSED: [u8; 6] = [0x2d, 0, 0, 0, 0, 0];
pub const KEY_X_RELEASED: [u8; 6] = [0xad, 0, 0, 0, 0, 0];
pub const KEY_C_PRESSED: [u8; 6] = [0x2e, 0, 0, 0, 0, 0];
pub const KEY_C_RELEASED: [u8; 6] = [0xae, 0, 0, 0, 0, 0];
pub const KEY_V_PRESSED: [u8; 6] = [0x2f, 0, 0, 0, 0, 0];
pub const KEY_V_RELEASED: [u8; 6] = [0xaf, 0, 0, 0, 0, 0];
pub const KEY_B_PRESSED: [u8; 6] = [0x30, 0, 0, 0, 0, 0];
pub const KEY_B_RELEASED: [u8; 6] = [0xb0, 0, 0, 0, 0, 0];
pub const KEY_N_PRESSED: [u8; 6] = [0x31, 0, 0, 0, 0, 0];
pub const KEY_N_RELEASED: [u8; 6] = [0xb1, 0, 0, 0, 0, 0];
pub const KEY_M_PRESSED: [u8; 6] = [0x32, 0, 0, 0, 0, 0];
pub const KEY_M_RELEASED: [u8; 6] = [0xb2, 0, 0, 0, 0, 0];
// <,
pub const KEY_COMMA_PRESSED: [u8; 6] = [0x33, 0, 0, 0, 0, 0];
pub const KEY_COMMA_RELEASED: [u8; 6] = [0xb3, 0, 0, 0, 0, 0];
// >.
pub const KEY_DOT_PRESSED: [u8; 6] = [0x34, 0, 0, 0, 0, 0];
pub const KEY_DOT_RELEASED: [u8; 6] = [0xb4, 0, 0, 0, 0, 0];
// ?/
pub const KEY_FORWARD_SLASH_PRESSED: [u8; 6] = [0x35, 0, 0, 0, 0, 0];
pub const KEY_FORWARD_SLASH_RELEASED: [u8; 6] = [0xb5, 0, 0, 0, 0, 0];
pub const KEY_RIGHT_SHIFT_PRESSED: [u8; 6] = [0x36, 0, 0, 0, 0, 0];
pub const KEY_RIGHT_SHIFT_RELEASED: [u8; 6] = [0xb6, 0, 0, 0, 0, 0];
pub const KEY_LEFT_CTRL_PRESSED: [u8; 6] = [0x1d, 0, 0, 0, 0, 0];
pub const KEY_LEFT_CTRL_RELEASED: [u8; 6] = [0x9d, 0, 0, 0, 0, 0];
pub const KEY_LEFT_ALT_PRESSED: [u8; 6] = [0x38, 0, 0, 0, 0, 0];
pub const KEY_LEFT_ALT_RELEASED: [u8; 6] = [0xb8, 0, 0, 0, 0, 0];
pub const KEY_SPACE_PRESSED: [u8; 6] = [0x39, 0, 0, 0, 0, 0];
pub const KEY_SPACE_RELEASED: [u8; 6] = [0xb9, 0, 0, 0, 0, 0];
pub const KEY_RIGHT_ALT_PRESSED: [u8; 6] = [0xe0, 0x38, 0, 0, 0, 0];
pub const KEY_RIGHT_ALT_RELEASED: [u8; 6] = [0xe0, 0xb8, 0, 0, 0, 0];
pub const KEY_RIGHT_CTRL_PRESSED: [u8; 6] = [0xe0, 0x1d, 0, 0, 0, 0];
pub const KEY_RIGHT_CTRL_RELEASED: [u8; 6] = [0xe0, 0x9d, 0, 0, 0, 0];

// 附加键盘及小键盘区
pub const KEY_PRINT_SCREEN_PRESSED: [u8; 6] = [0xe0, 0x2a, 0xe0, 0x37, 0, 0];
pub const KEY_PRINT_SCREEN_RELEASED: [u8; 6] = [0xe0, 0xb7, 0xe0, 0xaa, 0, 0];
pub const KEY_SCROLL_LOCK_PRESSED: [u8; 6] = [0x46, 0, 0, 0, 0, 0];
pub const KEY_SCROLL_LOCK_RELEASED: [u8; 6] = [0xc6, 0, 0, 0, 0, 0];
pub const KEY_PAUSE_BREAK_PRESSED: [u8; 6] = [0xe0, 0x1d, 0x45, 0xe1, 0x9d, 0xc5];
pub const KEY_INSERT_PRESSED: [u8; 6] = [0xe0, 0x52, 0, 0, 0, 0];
pub const KEY_INSERT_RELEASED: [u8; 6] = [0xe0, 0xd2, 0, 0, 0, 0];
pub const KEY_NUM_LOCK_PRESSED: [u8; 6] = [0x45, 0, 0, 0, 0, 0];
pub const KEY_NUM_LOCK_RELEASED: [u8; 6] = [0xc5, 0, 0, 0, 0, 0];
// /
pub const KEY_FORWARD_SLASH_EXT_PRESSED: [u8; 6] = [0xe0, 0x35, 0, 0, 0, 0];
pub const KEY_FORWARD_SLASH_EXT_RELEASED: [u8; 6] = [0xe0, 0xb5, 0, 0, 0, 0];
// *
pub const KEY_ASTERISK_PRESSED: [u8; 6] = [0x37, 0, 0, 0, 0, 0];
pub const KEY_ASTERISK_RELEASED: [u8; 6] = [0xb7, 0, 0, 0, 0, 0];
// -
pub const KEY_DASH_PRESSED: [u8; 6] = [0x4a, 0, 0, 0, 0, 0];
pub const KEY_DASH_RELEASED: [u8; 6] = [0xca, 0, 0, 0, 0, 0];
pub const KEY_HOME_PRESSED: [u8; 6] = [0xe0, 0x47, 0, 0, 0, 0];
pub const KEY_HOME_RELEASED: [u8; 6] = [0xe0, 0xc7, 0, 0, 0, 0];
pub const KEY_PAGE_UP_PRESSED: [u8; 6] = [0xe0, 0x49, 0, 0, 0, 0];
pub const KEY_PAGE_UP_RELEASED: [u8; 6] = [0xe0, 0xc9, 0, 0, 0, 0];
pub const KEY_DELETE_PRESSED: [u8; 6] = [0xe0, 0x53, 0, 0, 0, 0];
pub const KEY_DELETE_RELEASED: [u8; 6] = [0xe0, 0xd3, 0, 0, 0, 0];
pub const KEY_END_PRESSED: [u8; 6] = [0xe0, 0x4f, 0, 0, 0, 0];
pub const KEY_END_RELEASED: [u8; 6] = [0xe0, 0xcf, 0, 0, 0, 0];
pub const KEY_PAGE_DOWN_PRESSED: [u8; 6] = [0xe0, 0x51, 0, 0, 0, 0];
pub const KEY_PAGE_DOWN_RELEASED: [u8; 6] = [0xe0, 0xd1, 0, 0, 0, 0];
pub const KEY_LEFT_PRESSED: [u8; 6] = [0xe0, 0x46, 0, 0, 0, 0];
pub const KEY_LEFT_RELEASED: [u8; 6] = [0xe0, 0xc6, 0, 0, 0, 0];
pub const KEY_RIGHT_PRESSED: [u8; 6] = [0xe0, 0x4d, 0, 0, 0, 0];
pub const KEY_RIGHT_RELEASED: [u8; 6] = [0xe0, 0xcd, 0, 0, 0, 0];
pub const KEY_UP_PRESSED: [u8; 6] = [0xe0, 0x48, 0, 0, 0, 0];
pub const KEY_UP_RELEASED: [u8; 6] = [0xe0, 0xc8, 0, 0, 0, 0];
pub const KEY_DOWN_PRESSED: [u8; 6] = [0xe0, 0x50, 0, 0, 0, 0];
pub const KEY_DOWN_RELEASED: [u8; 6] = [0xe0, 0xd0, 0, 0, 0, 0];
pub const KEY_7HOME_PRESSED: [u8; 6] = [0x47, 0, 0, 0, 0, 0];
pub const KEY_7HOME_RELEASED: [u8; 6] = [0xc7, 0, 0, 0, 0, 0];
pub const KEY_8UP_PRESSED: [u8; 6] = [0x48, 0, 0, 0, 0, 0];
pub const KEY_8UP_RELEASED: [u8; 6] = [0xc8, 0, 0, 0, 0, 0];
pub const KEY_9_PG_UP_PRESSED: [u8; 6] = [0x49, 0, 0, 0, 0, 0];
pub const KEY_9_PG_UP_RELEASED: [u8; 6] = [0xc9, 0, 0, 0, 0, 0];
pub const KEY_4LEFT_PRESSED: [u8; 6] = [0x4b, 0, 0, 0, 0, 0];
pub const KEY_4LEFT_RELEASED: [u8; 6] = [0xcb, 0, 0, 0, 0, 0];
pub const KEY_5_EXT_PRESSED: [u8; 6] = [0x4c, 0, 0, 0, 0, 0];
pub const KEY_5_EXT_RELEASED: [u8; 6] = [0xcc, 0, 0, 0, 0, 0];
pub const KEY_6RIGHT_PRESSED: [u8; 6] = [0x4d, 0, 0, 0, 0, 0];
pub const KEY_6RIGHT_RELEASED: [u8; 6] = [0xcd, 0, 0, 0, 0, 0];
pub const KEY_1END_PRESSED: [u8; 6] = [0x4f, 0, 0, 0, 0, 0];
pub const KEY_1END_RELEASED: [u8; 6] = [0xcf, 0, 0, 0, 0, 0];
pub const KEY_2DOWN_PRESSED: [u8; 6] = [0x50, 0, 0, 0, 0, 0];
pub const KEY_2DOWN_RELEASED: [u8; 6] = [0xd0, 0, 0, 0, 0, 0];
pub const KEY_3_PG_DN_PRESSED: [u8; 6] = [0x51, 0, 0, 0, 0, 0];
pub const KEY_3_PG_DN_RELEASED: [u8; 6] = [0xd1, 0, 0, 0, 0, 0];
pub const KEY_0_INS_PRESSED: [u8; 6] = [0x52, 0, 0, 0, 0, 0];
pub const KEY_0_INS_RELEASED: [u8; 6] = [0xd2, 0, 0, 0, 0, 0];
// .del
pub const KEY_DEL_PRESSED: [u8; 6] = [0x53, 0, 0, 0, 0, 0];
pub const KEY_DEL_RELEASED: [u8; 6] = [0xd3, 0, 0, 0, 0, 0];
pub const KEY_PLUS_PRESSED: [u8; 6] = [0x4e, 0, 0, 0, 0, 0];
pub const KEY_PLUS_RELEASED: [u8; 6] = [0xce, 0, 0, 0, 0, 0];
pub const KEY_ENTER_EXT_PRESSED: [u8; 6] = [0xe0, 0x1c, 0, 0, 0, 0];
pub const KEY_ENTER_EXT_RELEASED: [u8; 6] = [0xe0, 0x9c, 0, 0, 0, 0];
