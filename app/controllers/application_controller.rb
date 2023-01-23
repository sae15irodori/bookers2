class ApplicationController < ActionController::Base
  before_action :authenticate_user!, except: [:top]
  before_action :configure_permitted_parameters, if: :devise_controller?
  #ユーザー認証・登録する前に(devise利用する前)↓のメソッド実行される

    def after_sign_in_path_for(resource)
      user_path(current_user.id)#ログイン後ﾕｰｻﾞｰのshowﾍﾟｰｼﾞに遷移
    end


    def after_sign_aout_path_for(resource)
      root_path#ログアウト後トップページへ遷移
    end
  protected

  def configure_permitted_parameters
    devise_parameter_sanitizer.permit(:sign_up, keys: [:email])#
  end
end
