----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    11:53:42 11/20/2021 
-- Design Name: 
-- Module Name:    DELATCH - Behavioral 
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

entity DELATCH is
    Port ( D : in  STD_LOGIC;
           E : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end DELATCH;

architecture Behavioral of DELATCH is
	signal QBuf : STD_LOGIC;
begin
	main: Process(D, E)
	begin
		if E = '1' then
			QBuf <= D;
		end if;
	end process;
	
	Q <= QBuf;
end Behavioral;

