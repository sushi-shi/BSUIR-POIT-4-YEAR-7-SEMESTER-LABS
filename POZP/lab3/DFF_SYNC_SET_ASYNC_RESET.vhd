----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:08:12 11/19/2021 
-- Design Name: 
-- Module Name:    DFF_SYNC_SET_ASYNC_RESET - Behavioral 
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

entity DFF_SYNC_SET_ASYNC_RESET is
Port ( D : in STD_LOGIC;
			 CLK : in STD_LOGIC;
			 CLR : in STD_LOGIC;
			 PRE : in STD_LOGIC;
			 Q   : out STD_LOGIC);
end DFF_SYNC_SET_ASYNC_RESET;

architecture Behavioral of DFF_SYNC_SET_ASYNC_RESET is
	signal Q_TMP: STD_LOGIC;
begin
	main: process(D, CLK, CLR, PRE)
	begin
		if (CLR = '1') then
			Q_TMP <= '0';
		elsif (rising_edge(CLK)) then
			if (PRE = '1') then
				Q_TMP <= '1';
			else
				Q_TMP <= D;
			end if;
		end if;
	end process;

	Q <= Q_TMP;
end Behavioral;

