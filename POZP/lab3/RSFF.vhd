----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    17:32:13 11/19/2021 
-- Design Name: 
-- Module Name:    RSFF - Behavioral 
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

entity RSFF is
Port (R : in STD_LOGIC;
		S : in STD_LOGIC;
		CLK : in STD_LOGIC;
		Q   : out STD_LOGIC);
end RSFF;

architecture Behavioral of RSFF is
signal Q_TMP: STD_LOGIC;
begin
	main: process(R, CLK, S)
	begin
		if rising_edge(CLK) then
			if S = '0' and R = '0' then
				Q_TMP <= Q_TMP;
			elsif S = '1' and R = '0' then
				Q_TMP <= '1';
			elsif S = '0' and R = '1' then
				Q_TMP <= '0';
				
			else 
				Q_TMP <= 'Z';
			end if;
		end if;
	end process;
		
	Q <= Q_TMP;

end Behavioral;

