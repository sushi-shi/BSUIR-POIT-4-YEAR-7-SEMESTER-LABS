----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    17:31:01 11/19/2021 
-- Design Name: 
-- Module Name:    JKFF - Behavioral 
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

entity TFF is
    Port ( T : in  STD_LOGIC;
           CLK : in  STD_LOGIC;
           CLR : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end TFF;

architecture Behavioral of TFF is
    signal Q_TMP : STD_LOGIC;
begin

	main: process (T, CLK, CLR)
		 begin
			  if CLR = '1' then
					Q_TMP <= '0';
			  elsif rising_edge(CLK) and T = '1' then
					Q_TMP <= not Q_TMP;
			  end if;
		 end process;
		 
	  Q <= Q_TMP;

end Behavioral;

