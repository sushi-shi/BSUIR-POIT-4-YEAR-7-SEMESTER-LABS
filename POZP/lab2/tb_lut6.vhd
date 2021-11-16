----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    18:27:31 11/16/2021 
-- Design Name: 
-- Module Name:    tb_lut6 - Behavioral 
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

entity tb_lut6 is
end tb_lut6;

architecture Behavioral of tb_lut6 is

signal addr :  STD_LOGIC_VECTOR (5 downto 0);
signal Q_BHVR :  STD_LOGIC;
signal Q_STR :  STD_LOGIC;

signal test_vector: std_logic_vector(  5 downto 0);

signal error: std_logic;

 constant period : time := 10 ns;

begin
   uut_beh: entity work.LUT6_beh PORT MAP (
		addr => addr,
		Q => Q_BHVR
	);

    uut_struct: entity work.LUT6_STR PORT MAP (
		addr => addr,
		Q => Q_STR
	);


addr <= test_vector;

   -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 127 loop
        test_vector <= std_logic_vector(to_unsigned(i, test_vector'length)); 
      wait for period;
    end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error <= Q_BHVR xor Q_STR;
end;

