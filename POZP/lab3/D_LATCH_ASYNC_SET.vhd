----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:30:59 11/19/2021 
-- Design Name: 
-- Module Name:    D_LATCH_ASYNC_SET - Behavioral 
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
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity DFF_ASYNC_SET is
Port ( D : in STD_LOGIC;
			 CLK : in STD_LOGIC;
			 PRE : in STD_LOGIC;
			 Q   : out STD_LOGIC);
end DFF_ASYNC_SET;

architecture Behavioral of DFF_ASYNC_SET is
	signal Q_TMP: STD_LOGIC;
begin
	main: process(D, CLK, PRE)
	begin
		if (PRE = '1') then
			Q_TMP <= '1';
		elsif (rising_edge(CLK)) then
			Q_TMP <= D;
		end if;
	end process;

	Q <= Q_TMP;


end Behavioral;

