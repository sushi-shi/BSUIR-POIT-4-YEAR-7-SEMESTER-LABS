----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:04:49 11/16/2021 
-- Design Name: 
-- Module Name:    tb_and5 - Behavioral 
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
USE ieee.numeric_std.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_and5 is
end tb_and5;

architecture Behavioral of tb_and5 is
   signal A :   STD_LOGIC_VECTOR (4 downto 0);
   signal S :   STD_LOGIC;
	signal S_str :   STD_LOGIC;

  
  signal error: std_logic;
  
  signal test_vector: std_logic_vector (4 downto 0);
  
  constant period : time := 10 ns;
begin
   uut_beh: entity work.AND_5 PORT MAP (
		A => A,
		S => S
    );

    uut_struct: entity work.AND5_STRUCT PORT MAP (
          A => A,
			 S => S_str
     );
	  A <= test_vector;

  -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 31 loop
      
        test_vector <= std_logic_vector(to_unsigned(i, test_vector'length)); 
      wait for period;
    end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error <= S xor S_Str;
END;


