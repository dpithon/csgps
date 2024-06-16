/is_odd { dup 2 mod 1 eq } def
/is_even { dup 2 mod 0 eq } def
/make_even { is_odd { 1 add } if } def

%57 is_odd { 1 add } if = clear 
%57 is_even { 1 add } if = clear 

41 make_even = clear
% 42 make_even =
