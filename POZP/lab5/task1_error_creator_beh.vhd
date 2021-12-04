----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:00:43 PM
-- Design Name: 
-- Module Name: task1_error_creator_beh - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
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
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity MkError is
    generic(bits: integer := 4);
    Port (
        Qin: in STD_LOGIC_VECTOR(0 to bits);
		  
        Qout: out STD_LOGIC_VECTOR(0 to bits)
    );
end MkError;

architecture Behavioral of MkError is
    signal q_t: STD_LOGIC_VECTOR(0 to bits);
begin

    main: process(Qin)
    begin
        q_t <= not Qin(0) & Qin(1 to bits);
    end process;
	 
    Qout <= q_t;
	 
end Behavioral;
