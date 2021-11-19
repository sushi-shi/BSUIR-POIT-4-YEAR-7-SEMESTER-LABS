----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:41:37 11/19/2021 
-- Design Name: 
-- Module Name:    MY_AND - Behavioral 
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

entity MY_AND is
    Port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end MY_AND;

architecture Behavioral of MY_AND is

begin
	Q <= A and B;

end Behavioral;

