----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    16:31:22 11/16/2021 
-- Design Name: 
-- Module Name:    SUM_2_Beh - Behavioral 
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

entity SUM_2_Beh is
    Port ( X : in  STD_LOGIC_VECTOR (1 downto 0);
           Y : in  STD_LOGIC_VECTOR (1 downto 0);
           S : out  STD_LOGIC_VECTOR (1 downto 0);
			  P : out STD_LOGIC);
end SUM_2_Beh;

architecture Behavioral of SUM_2_Beh is

signal tmp0, tmp1, tmp2, tmp3 : STD_LOGIC;

begin
 S(0) <= X(0) xor Y(0);
 tmp0 <= X(0) and Y(0);
 
 tmp1 <= X(1) xor Y(1);
 tmp2 <= X(1) and Y(1);
 
 S(1) <= tmp0 xor tmp1;
 P <= tmp2 or (tmp0 and tmp1);
 
end Behavioral;

