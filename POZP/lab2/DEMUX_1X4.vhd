----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:10:32 11/16/2021 
-- Design Name: 
-- Module Name:    DEMUX_1X4 - Behavioral 
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

entity DEMUX_1X4 is
    Port ( X : in  STD_LOGIC;
           S : in  STD_LOGIC_VECTOR (1 downto 0);
           Z : out  STD_LOGIC_VECTOR (3 downto 0));
end DEMUX_1X4;

architecture Behavioral of DEMUX_1X4 is

begin
	Z(0) <= X and not S(0) and not S(1);
	Z(1) <= X and S(0) and not S(1);
	Z(2) <= X and not S(0) and S(1);
	Z(3) <= X and s(0) and S(1);

end Behavioral;

