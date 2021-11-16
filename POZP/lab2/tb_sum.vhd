----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:35:25 11/16/2021 
-- Design Name: 
-- Module Name:    tb_sum - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_sum is
end tb_sum;

architecture Behavioral of tb_sum is

signal X : STD_LOGIC_VECTOR (1 downto 0);
signal Y :   STD_LOGIC_VECTOR (1 downto 0);
signal S :  STD_LOGIC_VECTOR (1 downto 0);
signal P :  STD_LOGIC;

signal S_STR :  STD_LOGIC_VECTOR (1 downto 0);
signal P_STR :  STD_LOGIC;


signal test_vector: std_logic_vector (3 downto 0);

signal error: std_logic;

 constant period : time := 10 ns;

begin
   uut_beh: entity work.SUM_2_Beh PORT MAP (
		X => X,
		Y => Y,
		S => S,
		P => P
	);

    uut_struct: entity work.SUM_2_STR PORT MAP (
		X => X,
		Y => Y,
		S => S_STR,
		P => P_STR
	);


X(0) <= test_vector(0);
X(1) <= test_vector(1);
Y(0) <= test_vector(2);
Y(1) <= test_vector(3);

   -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 15 loop
      
        test_vector <= std_logic_vector(to_unsigned(i, test_vector'length)); 
      wait for period;
    end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error <= (P xor P_STR) or (S(0) xor S_STR(0)) or (S(1) xor S_STR(1));
end;
