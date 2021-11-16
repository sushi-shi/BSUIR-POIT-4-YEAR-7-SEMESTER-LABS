----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    19:33:02 11/02/2021 
-- Design Name: 
-- Module Name:    mux4 - Behavioral 
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

entity mux4 is
    Port ( a1 : in  STD_LOGIC;
           a2 : in  STD_LOGIC;
           a3 : in  STD_LOGIC;
           a4 : in  STD_LOGIC;
           sel : in  STD_LOGIC_VECTOR(1 downto 0);
           b : out  STD_LOGIC);
end mux4;

architecture Behavioral of mux4 is
begin
p_mux : process(a1, a2, a3, a4, sel)
begin
	case sel is 
		when "00" => b <= a1;
		when "01" => b <= a2;
		when "10" => b <= a3;
		when others => b <= a4;
	end case;
end process p_mux;


end Behavioral;

