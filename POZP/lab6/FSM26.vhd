----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    22:42:17 11/20/2021 
-- Design Name: 
-- Module Name:    FSM26 - Behavioral 
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

entity FSM26 is
    port (Clk : in std_logic;
          RST : in std_logic;
          IP : in std_logic_vector(3 downto 0);
          DataOut : out std_logic_vector(1 downto 0));
end FSM26;

architecture Behavioral of FSM26 is

type states is (S0, S1, S2, S3, S4);

signal cur_state : states;
signal next_state : states;

signal output : std_logic_vector(1 downto 0);

begin

    memory : process(Clk, RST, next_state)
    begin
        if (RST = '1') then
            cur_state <= S0;
        elsif (rising_edge(Clk)) then
            cur_state <= next_state;
        end if;
    end process;

    next_state_generator : process(cur_state, IP)
    begin
        case cur_state is
            when S0 => if (IP = "0000") then
                           next_state <= S2;
                       else
                           next_state <= S0;
                       end if;
            when S1 => next_state <= S1;
            when S2 => if (IP = "0001") then
                           next_state <= S1;
                       elsif (IP = "0010") then
                           next_state <= S4;
                       elsif (IP = "0100") then
                           next_state <= S3;
                       else
                           next_state <= S2;
                       end if;
            when S3 => next_state <= S3;
            when S4 => if (IP = "0011") then
                           next_state <= S2;
                       else
                           next_state <= S4;
                       end if;
            when others => next_state <= S0;
        end case;
    end process;

    output_generator : process(cur_state)
    begin
        case cur_state is
            when S0 | S3 => output <= "00";
            when S1 => output <= "01";
            when S2 => output <= "10";
            when S4 => output <= "11";
            when others => output <= "00";
        end case;
    end process;

    DataOut <= output;


end Behavioral;

